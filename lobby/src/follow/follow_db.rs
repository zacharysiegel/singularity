use shared::error::AppError;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use super::follow_model::{FollowEntity, FollowSummary};

pub async fn create_follow(
    pool: &PgPool,
    source_account_id: Uuid,
    target_account_id: Uuid,
) -> Result<FollowEntity, AppError> {
    let record: FollowEntity = sqlx::query_as!(
        FollowEntity,
        "insert into follow (source_account_id, target_account_id)
         values ($1, $2)
         returning *",
        source_account_id,
        target_account_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn delete_follow(
    pool: &PgPool,
    source_account_id: Uuid,
    target_account_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query!(
        "delete from follow
         where source_account_id = $1 and target_account_id = $2",
        source_account_id,
        target_account_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_follow(
    pool: &PgPool,
    source_account_id: Uuid,
    target_account_id: Uuid,
) -> Result<Option<FollowEntity>, AppError> {
    let record: Option<FollowEntity> = sqlx::query_as!(
        FollowEntity,
        "select follow.source_account_id, follow.target_account_id, follow.created
         from follow
         where follow.source_account_id = $1 and follow.target_account_id = $2",
        source_account_id,
        target_account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_followers(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<FollowSummary>, AppError> {
    let records: Vec<FollowSummary> = sqlx::query_as!(
        FollowSummary,
        "select
             follow.source_account_id as account_id,
             account.username,
             exists(
                 select 1 from mutual_follow_view
                 where mutual_follow_view.account_id = $1
                   and mutual_follow_view.mutual_account_id = follow.source_account_id
             ) as \"is_mutual!\"
         from follow
         inner join account on account.id = follow.source_account_id
         where follow.target_account_id = $1
           and account.deleted_at is null
         order by follow.created desc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_following(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<FollowSummary>, AppError> {
    let records: Vec<FollowSummary> = sqlx::query_as!(
        FollowSummary,
        "select
             follow.target_account_id as account_id,
             account.username,
             exists(
                 select 1 from mutual_follow_view
                 where mutual_follow_view.account_id = $1
                   and mutual_follow_view.mutual_account_id = follow.target_account_id
             ) as \"is_mutual!\"
         from follow
         inner join account on account.id = follow.target_account_id
         where follow.source_account_id = $1
           and account.deleted_at is null
         order by follow.created desc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_mutuals(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<FollowSummary>, AppError> {
    let records: Vec<FollowSummary> = sqlx::query_as!(
        FollowSummary,
        "select
             mutual_follow_view.mutual_account_id as \"account_id!\",
             account.username as \"username!\",
             true as \"is_mutual!\"
         from mutual_follow_view
         inner join account on account.id = mutual_follow_view.mutual_account_id
         where mutual_follow_view.account_id = $1
           and account.deleted_at is null
         order by account.username asc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn delete_follows_by_account<'e, E: sqlx::Executor<'e, Database = Postgres>>(
    executor: E,
    account_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query!(
        "delete from follow
         where source_account_id = $1 or target_account_id = $1",
        account_id,
    )
    .execute(executor)
    .await?;
    Ok(())
}
