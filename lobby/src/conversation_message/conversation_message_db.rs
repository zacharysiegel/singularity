use chrono::{DateTime, Utc};
use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_message_model::ConversationMessageRow;

pub async fn create_message(
    pool: &PgPool,
    conversation_id: Uuid,
    sender_account_id: Uuid,
    content: &str,
) -> Result<ConversationMessageRow, AppError> {
    let record: ConversationMessageRow = sqlx::query_as!(
        ConversationMessageRow,
        "with inserted as (
             insert into conversation_message (conversation_id, sender_account_id, content)
             values ($1, $2, $3)
             returning *
         )
         select inserted.id, inserted.conversation_id, inserted.sender_account_id, inserted.content,
                (conversation_member.exited is not null) as \"sender_anonymized!\",
                inserted.created
         from inserted
         inner join conversation_member on conversation_member.conversation_id = inserted.conversation_id
                                       and conversation_member.account_id = inserted.sender_account_id",
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
) -> Result<Vec<ConversationMessageRow>, AppError> {
    let message_rows: Vec<ConversationMessageRow> = sqlx::query_as!(
        ConversationMessageRow,
        "select conversation_message.id, conversation_message.conversation_id,
                conversation_message.sender_account_id, conversation_message.content,
                (conversation_member.exited is not null) as \"sender_anonymized!\",
                conversation_message.created
         from conversation_message
         inner join conversation_member on conversation_member.conversation_id = conversation_message.conversation_id
                                       and conversation_member.account_id = conversation_message.sender_account_id
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
    Ok(message_rows)
}
