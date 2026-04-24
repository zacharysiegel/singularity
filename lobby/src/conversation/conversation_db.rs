use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_model::{ConversationEntity, ConversationMemberEntity};

pub async fn create_conversation(
    pool: &PgPool,
    name: Option<&str>,
    game_id: Option<Uuid>,
) -> Result<ConversationEntity, AppError> {
    let record: ConversationEntity = sqlx::query_as!(
        ConversationEntity,
        "insert into conversation (name, game_id)
         values ($1, $2)
         returning *",
        name,
        game_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_conversation_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<ConversationEntity>, AppError> {
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

pub async fn leave_conversation(
    pool: &PgPool,
    conversation_id: Uuid,
    account_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query!(
        "update conversation_member
         set exited = now()
         where conversation_id = $1
           and account_id = $2
           and exited is null",
        conversation_id,
        account_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_active_member_count(
    pool: &PgPool,
    conversation_id: Uuid,
) -> Result<i64, AppError> {
    /* count(*) returns a nullable type. Even though count(*) in SQL never actually returns NULL,
        SQLx's compile-time analysis can't prove that, so it conservatively marks aggregate function
        results as Option<i64>. */
    let record = sqlx::query!(r#"
        select count(*) as "count!"
        from conversation_member
        where conversation_member.conversation_id = $1
            and conversation_member.exited is null
        "#,
        conversation_id,
    )
        .fetch_one(pool)
        .await?;
    Ok(record.count)
}

pub async fn delete_conversation(pool: &PgPool, conversation_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "delete from conversation
         where id = $1",
        conversation_id,
    )
    .execute(pool)
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

pub async fn get_active_conversation_ids_by_account(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<Uuid>, AppError> {
    let conversation_id_records = sqlx::query!(
        "select conversation_member.conversation_id
         from conversation_member
         where conversation_member.account_id = $1
           and conversation_member.exited is null",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    let conversation_ids: Vec<Uuid> = conversation_id_records
        .into_iter()
        .map(|record| record.conversation_id)
        .collect();
    Ok(conversation_ids)
}
