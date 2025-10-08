use crate::error::AppErrorStatic;
use crate::network::connection::{Connection, ConnectionReader, ConnectionWriter};
use crate::network::frame_buffer::FrameBuffer;
use crate::network::protocol::Frame;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};
use tokio::time;

// todo: if this function is the only user of ConnectionReader, it can be given full ownership and obviate the lock acquisition
//  It's worth trying to find a way to have single ownership of the ConnectionWriter as well
pub async fn monitor_incoming_frames<F, Fut>(connection: Arc<Connection>, callback: F)
where
    F: Fn(Arc<Connection>, Frame) -> Fut,
    Fut: Future<Output = ()>,
{
    let mut reader: RwLockWriteGuard<ConnectionReader> = connection.reader.write().await;
    loop {
        let Ok(_) = reader.read_chunk().await else {
            log::info!("Connection terminated; {:?}", connection);
            break;
        };

        match reader.buffer.pop_frames() {
            Ok(frames) => {
                for frame in frames {
                    callback(connection.clone(), frame).await;
                }
            }
            Err(e) => {
                log::error!("Failed to read from TCP stream; {:#}", e);
                break;
            }
        }
    }
}

pub async fn monitor_outgoing_frames(connection: Arc<Connection>) -> Result<(), AppErrorStatic> {
    loop {
        let writer: RwLockReadGuard<ConnectionWriter> = connection.writer.read().await;
        if writer.buffer.used_space() == 0 {
            time::sleep(Duration::from_millis(50)).await;
            continue;
        }
        drop(writer);

        let mut writer: RwLockWriteGuard<ConnectionWriter> = connection.writer.write().await;
        let frames: Vec<Frame> = writer.buffer.pop_frames()?;
        for frame in frames {
            writer.write_frame(&frame).await?;
        }
    }
}
