use crate::error::AppErrorStatic;
use crate::srtp::connection::{BUFFER_SIZE, BytesRead, ConnectionReader, ConnectionWriter, WriteBufferT};
use crate::srtp::frame_buffer::FrameBuffer;
use crate::srtp::protocol::Frame;
use crate::srtp::ring_buffer::RingBuffer;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLockReadGuard, RwLockWriteGuard};
use tokio::time;

pub async fn monitor_incoming_frames<F, Fut>(mut reader: ConnectionReader, callback: F)
where
    F: Fn(WriteBufferT, Frame) -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let bytes_read: BytesRead = match reader.read_chunk().await {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                log::error!("Connection read error; {:?} {:#}", reader, error);
                break;
            }
        };

        match bytes_read {
            BytesRead::ReadClosed => {
                log::info!("Connection closed; {:?}", reader);
                break;
            }
            BytesRead::Some(_) => {}
        }

        match reader.read_buffer.pop_frames() {
            Ok(frames) => {
                for frame in frames {
                    callback(reader.write_buffer.clone(), frame).await;
                }
            }
            Err(e) => {
                log::error!("Failed to read from TCP stream; {:#}", e);
                break;
            }
        }
    }
}

pub async fn monitor_outgoing_frames(
    mut writer: ConnectionWriter,
    shutdown: Arc<Notify>,
) -> Result<(), AppErrorStatic> {
    loop {
        let buffer_g: RwLockReadGuard<RingBuffer<u8, { BUFFER_SIZE }>> = writer.buffer.read().await;
        if buffer_g.used_space() == 0 {
            drop(buffer_g);
            tokio::select! {
                _ = time::sleep(Duration::from_millis(50)) => continue,
                _ = shutdown.notified() => break,
            }
        }
        drop(buffer_g);

        let mut buffer_g: RwLockWriteGuard<RingBuffer<u8, { BUFFER_SIZE }>> = writer.buffer.write().await;
        let frames: Vec<Frame> = buffer_g.pop_frames()?;
        drop(buffer_g);

        for frame in frames {
            writer.write_frame(&frame).await?;
        }
    }
    Ok(())
}
