use crate::error::AppErrorStatic;
use crate::network::connection::{ConnectionReader, ConnectionWriter};
use crate::network::frame_buffer::FrameBuffer;
use crate::network::protocol::Frame;
use crate::network::ring_buffer::RingBuffer;
use std::future::Future;
use std::time::Duration;
use tokio::sync::{RwLockReadGuard, RwLockWriteGuard};
use tokio::time;

pub async fn monitor_incoming_frames<F, Fut>(mut reader: ConnectionReader, callback: F)
where
    F: Fn(Frame) -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        let Ok(_) = reader.read_chunk().await else {
            log::info!("Connection terminated; {:?}", reader);
            break;
        };

        match reader.buffer.pop_frames() {
            Ok(frames) => {
                for frame in frames {
                    callback(frame).await;
                }
            }
            Err(e) => {
                log::error!("Failed to read from TCP stream; {:#}", e);
                break;
            }
        }
    }
}

pub async fn monitor_outgoing_frames(mut writer: ConnectionWriter) -> Result<(), AppErrorStatic> {
    loop {
        let buffer_p = writer.buffer.clone();

        let buffer_l: RwLockReadGuard<RingBuffer<u8, 4096>> = buffer_p.read().await;
        if buffer_l.used_space() == 0 {
            time::sleep(Duration::from_millis(50)).await;
            continue;
        }
        drop(buffer_l);

        let mut buffer_l: RwLockWriteGuard<RingBuffer<u8, 4096>> = buffer_p.write().await;
        let frames: Vec<Frame> = buffer_l.pop_frames()?;
        drop(buffer_l);

        for frame in frames {
            writer.write_frame(&frame).await?;
        }
    }
}
