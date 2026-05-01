use std::fmt::Display;
use std::future::Future;
use std::time::Duration;

pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const CONTENT_TYPE_MSGPACK: &str = "application/msgpack";

const RETRY_BACKOFF_INITIAL: Duration = Duration::from_millis(200);
const RETRY_BACKOFF_MAX: Duration = Duration::from_secs(2);

pub async fn with_retry<F, Fut, T, E>(
    max_attempts: u32,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Display,
{
    let mut backoff: Duration = RETRY_BACKOFF_INITIAL;
    let mut last_error: Option<E> = None;

    for attempt in 1..=max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt < max_attempts {
                    log::warn!(
                        "Retrying; [attempt {attempt}/{max_attempts}] [{error}]"
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(RETRY_BACKOFF_MAX);
                }
                last_error = Some(error);
            }
        }
    }

    Err(last_error.unwrap())
}
