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
    // Atomic leave + auto-delete: the CTE updates the membership, counts remaining
    // active members, and deletes the conversation if none remain — all in one statement.
    // Returns the number of rows updated by the leave (0 or 1) so we know if it happened.
    let record = sqlx::query!(
        r#"
        with leave as (
            update conversation_member
            set exited = now()
            where conversation_id = $1 and account_id = $2 and exited is null
            returning conversation_id
        ),
        remaining as (
            select count(*) as active_count
            from conversation_member
            where conversation_id = $1 and exited is null
        ),
        cleanup as (
            delete from conversation
            where id = $1
              and exists (select 1 from leave)
              and (select active_count from remaining) = 0
        )
        select count(*) as "did_leave!" from leave
        "#,
        conversation_id,
        account_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(record.did_leave > 0)
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

pub async fn get_conversations_by_account_unsorted(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<ConversationEntity>, AppError> {
    let conversation_entities: Vec<ConversationEntity> = sqlx::query_as!(
        ConversationEntity,
        "select conversation.id, conversation.game_id, conversation.name, conversation.created
         from conversation
         inner join conversation_member on conversation_member.conversation_id = conversation.id
         where conversation_member.account_id = $1
           and conversation_member.exited is null",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(conversation_entities)
}
