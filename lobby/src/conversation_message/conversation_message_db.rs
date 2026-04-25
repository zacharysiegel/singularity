use chrono::{DateTime, Utc};
use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_message_model::ConversationMessageEntity;

pub async fn create_message(
    pool: &PgPool,
    conversation_id: Uuid,
    sender_account_id: Uuid,
    content: &str,
) -> Result<ConversationMessageEntity, AppError> {
    let record: ConversationMessageEntity = sqlx::query_as!(
        ConversationMessageEntity,
        "insert into conversation_message (conversation_id, sender_account_id, content)
         values ($1, $2, $3)
         returning *",
        conversation_id,
        sender_account_id,
        content,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_messages_by_conversation(
    pool: &PgPool,
    conversation_id: Uuid,
    limit: i64,
    before: Option<DateTime<Utc>>,
) -> Result<Vec<ConversationMessageEntity>, AppError> {
    let message_entities: Vec<ConversationMessageEntity> = sqlx::query_as!(
        ConversationMessageEntity,
        "select conversation_message.id,
               conversation_message.conversation_id,
               conversation_message.sender_account_id,
               conversation_message.content,
               conversation_message.created
         from conversation_message
         where conversation_message.conversation_id = $1
           and ($2::timestamptz is null or conversation_message.created < $2)
         order by conversation_message.created desc
         limit $3",
        conversation_id,
        before,
        limit,
    )
    .fetch_all(pool)
    .await?;
    Ok(message_entities)
}
