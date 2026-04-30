use shared::error::AppError;
use sqlx::postgres::PgQueryResult;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::conversation_model::{ConversationEntity, ConversationMemberEntity};

pub async fn create_conversation(
    pool: &PgPool,
    name: Option<&str>,
    game_id: Option<Uuid>,
    creator_account_id: Uuid,
    member_account_ids: &[Uuid],
) -> Result<ConversationEntity, AppError> {
    let mut transaction: Transaction<Postgres> = pool.begin().await?;

    let conversation: ConversationEntity = sqlx::query_as!(
        ConversationEntity,
        "insert into conversation (name, game_id)
         values ($1, $2)
         returning *",
        name,
        game_id,
    )
    .fetch_one(&mut *transaction)
    .await?;

    sqlx::query!(
        "insert into conversation_member (conversation_id, account_id)
         values ($1, $2)",
        conversation.id,
        creator_account_id,
    )
    .execute(&mut *transaction)
    .await?;

    for member_account_id in member_account_ids {
        if *member_account_id == creator_account_id {
            continue;
        }

        sqlx::query!(
            "insert into conversation_member (conversation_id, account_id)
             values ($1, $2)",
            conversation.id,
            *member_account_id,
        )
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;
    Ok(conversation)
}

pub async fn get_conversation_by_id(pool: &PgPool, id: Uuid) -> Result<Option<ConversationEntity>, AppError> {
    let record: Option<ConversationEntity> = sqlx::query_as!(
        ConversationEntity,
        "select conversation.id, conversation.game_id, conversation.name, conversation.created
         from conversation
         where conversation.id = $1",
        id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn add_member(
    pool: &PgPool,
    conversation_id: Uuid,
    account_id: Uuid,
) -> Result<ConversationMemberEntity, AppError> {
    let record: ConversationMemberEntity = sqlx::query_as!(
        ConversationMemberEntity,
        "insert into conversation_member (conversation_id, account_id)
         values ($1, $2)
         returning *",
        conversation_id,
        account_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_member(
    pool: &PgPool,
    conversation_id: Uuid,
    account_id: Uuid,
) -> Result<Option<ConversationMemberEntity>, AppError> {
    let record: Option<ConversationMemberEntity> = sqlx::query_as!(
        ConversationMemberEntity,
        "select conversation_member.conversation_id, conversation_member.account_id,
               conversation_member.entered, conversation_member.exited
         from conversation_member
         where conversation_member.conversation_id = $1
           and conversation_member.account_id = $2
           and conversation_member.exited is null",
        conversation_id,
        account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn leave_conversation(pool: &PgPool, conversation_id: Uuid, account_id: Uuid) -> Result<bool, AppError> {
    let mut transaction: Transaction<Postgres> = pool.begin().await?;
    let did_leave: bool = leave_conversation_exec(&mut *transaction, conversation_id, account_id).await?;
    if did_leave {
        delete_conversation_if_empty_exec(&mut *transaction, conversation_id).await?;
    }
    transaction.commit().await?;
    Ok(did_leave)
}

pub async fn leave_conversation_exec<'e, E: sqlx::Executor<'e, Database = Postgres>>(
    executor: E,
    conversation_id: Uuid,
    account_id: Uuid,
) -> Result<bool, AppError> {
    let leave_result: PgQueryResult = sqlx::query!(
        "update conversation_member
         set exited = now()
         where conversation_id = $1 and account_id = $2 and exited is null",
        conversation_id,
        account_id,
    )
    .execute(executor)
    .await?;
    Ok(leave_result.rows_affected() > 0)
}

pub async fn delete_conversation_if_empty_exec<'e, E: sqlx::Executor<'e, Database = Postgres>>(
    executor: E,
    conversation_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query!(
        "
        with active_members as (
            select 1 from conversation_member
            where conversation_id = $1 and exited is null
        )
        delete from conversation
        where id = $1
            and not exists (select 1 from active_members)",
        conversation_id,
    )
    .execute(executor)
    .await?;
    Ok(())
}

pub async fn get_conversations_by_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<ConversationEntity>, AppError> {
    let conversation_entities: Vec<ConversationEntity> = sqlx::query_as!(
        ConversationEntity,
        r#"
        select conversation.id, conversation.game_id, conversation.name, conversation.created
        from conversation
        inner join conversation_member on conversation_member.conversation_id = conversation.id
        left join conversation_latest_message_view on conversation_latest_message_view.conversation_id = conversation.id
        where conversation_member.account_id = $1
            and conversation_member.exited is null
        order by coalesce(conversation_latest_message_view.latest_message_created, conversation.created) desc
        "#,
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(conversation_entities)
}

pub async fn get_conversations_by_game_and_account(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Vec<ConversationEntity>, AppError> {
    let conversation_entities: Vec<ConversationEntity> = sqlx::query_as!(
        ConversationEntity,
        r#"
        select conversation.id, conversation.game_id, conversation.name, conversation.created
        from conversation
        inner join conversation_member on conversation_member.conversation_id = conversation.id
        where conversation.game_id = $1
            and conversation_member.account_id = $2
            and conversation_member.exited is null
        order by conversation.created asc
        "#,
        game_id,
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(conversation_entities)
}

/// Checks if a conversation already exists with exactly the given member set.
/// For in-game conversations, pass Some(game_id). For global conversations, pass None.
/// Uses symmetric set difference (double EXCEPT) to compare member sets.
pub async fn conversation_with_members_exists(
    pool: &PgPool,
    game_id: Option<Uuid>,
    member_account_ids: &[Uuid],
) -> Result<bool, AppError> {
    let record = sqlx::query!(
        r#"
        select conversation.id
        from conversation
        where
            ((conversation.game_id is null and $1::uuid is null) or conversation.game_id = $1)
            and not exists (
                select conversation_member.account_id from conversation_member
                where conversation_member.conversation_id = conversation.id
                    and conversation_member.exited is null
                except
                select unnest($2::uuid[])
            )
            and not exists (
                select unnest($2::uuid[])
                except
                select conversation_member.account_id from conversation_member
                where conversation_member.conversation_id = conversation.id
                    and conversation_member.exited is null
            )
        limit 1
        "#,
        game_id as Option<Uuid>,
        member_account_ids,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record.is_some())
}

pub async fn get_active_members(
    pool: &PgPool,
    conversation_id: Uuid,
) -> Result<Vec<ConversationMemberEntity>, AppError> {
    let member_records: Vec<ConversationMemberEntity> = sqlx::query_as!(
        ConversationMemberEntity,
        "
        select *
        from conversation_member
        where conversation_member.conversation_id = $1
            and conversation_member.exited is null
        ",
        conversation_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(member_records)
}
