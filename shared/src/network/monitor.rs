use std::future::Future;
use std::sync::Arc;
use tokio::sync::{RwLockWriteGuard};
use crate::network::connection::{Connection, ConnectionReader};
use crate::network::protocol::Frame;

pub async fn monitor_incoming_frames<F, Fut>(connection: Arc<Connection>, callback: F)
where
    F: Fn(Arc<Connection>, Frame) -> Fut,
    Fut: Future<Output = ()>,
{
    let mut reader: RwLockWriteGuard<ConnectionReader> = connection.reader.write().await;
    loop {
        match reader.read_frames().await {
            Ok(frames_o) => match frames_o {
                Some(frames) => {
                    for frame in frames {
                        callback(connection.clone(), frame).await;
                    }
                }
                None => {
                    log::info!("Connection terminated; {:?}", reader);
                    break;
                }
            },
            Err(e) => {
                log::error!("Failed to read from TCP stream; {:#}", e);
                break;
            }
        }
    }
}
