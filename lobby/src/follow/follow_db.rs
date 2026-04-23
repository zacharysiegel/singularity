use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::follow_model::{FollowEntity, FollowingSummaryRow};

pub async fn create_follow(
    pool: &PgPool,
    follower_account_id: Uuid,
    followed_account_id: Uuid,
) -> Result<FollowEntity, AppError> {
    let record: FollowEntity = sqlx::query_as!(
        FollowEntity,
        "insert into follow (follower_account_id, followed_account_id)
         values ($1, $2)
         returning *",
        follower_account_id,
        followed_account_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn delete_follow(
    pool: &PgPool,
    follower_account_id: Uuid,
    followed_account_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query!(
        "delete from follow
         where follower_account_id = $1 and followed_account_id = $2",
        follower_account_id,
        followed_account_id,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_follow(
    pool: &PgPool,
    follower_account_id: Uuid,
    followed_account_id: Uuid,
) -> Result<Option<FollowEntity>, AppError> {
    let record: Option<FollowEntity> = sqlx::query_as!(
        FollowEntity,
        "select follow.follower_account_id, follow.followed_account_id, follow.created
         from follow
         where follow.follower_account_id = $1 and follow.followed_account_id = $2",
        follower_account_id,
        followed_account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_followers(
    pool: &PgPool,
    account_id: Uuid,
) -> Result<Vec<FollowingSummaryRow>, AppError> {
    let records: Vec<FollowingSummaryRow> = sqlx::query_as!(
        FollowingSummaryRow,
        "select
             follow.follower_account_id as account_id,
             account.username,
             exists(
                 select 1 from mutual_follow_view
                 where mutual_follow_view.account_id = $1
                   and mutual_follow_view.mutual_account_id = follow.follower_account_id
             ) as \"is_mutual!\"
         from follow
         inner join account on account.id = follow.follower_account_id
         where follow.followed_account_id = $1
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
) -> Result<Vec<FollowingSummaryRow>, AppError> {
    let records: Vec<FollowingSummaryRow> = sqlx::query_as!(
        FollowingSummaryRow,
        "select
             follow.followed_account_id as account_id,
             account.username,
             exists(
                 select 1 from mutual_follow_view
                 where mutual_follow_view.account_id = $1
                   and mutual_follow_view.mutual_account_id = follow.followed_account_id
             ) as \"is_mutual!\"
         from follow
         inner join account on account.id = follow.followed_account_id
         where follow.follower_account_id = $1
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
) -> Result<Vec<FollowingSummaryRow>, AppError> {
    let records: Vec<FollowingSummaryRow> = sqlx::query_as!(
        FollowingSummaryRow,
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

pub async fn delete_follows_by_account(pool: &PgPool, account_id: Uuid) -> Result<(), AppError> {
    sqlx::query!(
        "delete from follow
         where follower_account_id = $1 or followed_account_id = $1",
        account_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
