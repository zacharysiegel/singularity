use std::future::Future;
use std::sync::Arc;
use tokio::sync::{RwLockWriteGuard};
use crate::network::connection::{Connection, ConnectionReader};
use crate::network::protocol::Frame;

// todo: if this function is the only user of ConnectionReader, it can be given full ownership and obviate the lock acquisition
//  It's worth trying to find a way to have single ownership of the ConnectionWriter as well
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
                    log::info!("Connection terminated; {:?}", connection);
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
