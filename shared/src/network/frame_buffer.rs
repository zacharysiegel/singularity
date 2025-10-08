use crate::error::{AppError, AppErrorStatic};
use crate::network::protocol::{Frame, Head, OperationType};
use crate::network::ring_buffer::{RingBuffer, RingBufferView};

pub trait FrameBuffer {
    fn pop_frames(&mut self) -> Result<Vec<Frame>, AppErrorStatic>;
    fn peek_frame_head(&self) -> Result<Option<Head>, AppError>;
    fn pop_frame_data(&mut self, head: &Head) -> Result<Vec<u8>, AppError>;
}

impl<const N: usize> FrameBuffer for RingBuffer<u8, N> {
    fn pop_frames(&mut self) -> Result<Vec<Frame>, AppErrorStatic> {
        let mut frames: Vec<Frame> = Vec::new();
        loop {
            let bytes_remaining: usize = self.used_space();
            let Some(head) = self.peek_frame_head()? else {
                break;
            };
            if head.length > bytes_remaining {
                break;
            }

            let frame_data: Vec<u8> = self.pop_frame_data(&head)?;
            let frame: Frame = Frame { head, data: frame_data };
            frames.push(frame);
        }
        Ok(frames)
    }

    fn peek_frame_head(&self) -> Result<Option<Head>, AppError> {
        if self.used_space() < 1 {
            return Ok(None);
        }
        let op_code_view: RingBufferView<u8> = self.peek(1)?; // Must be modified if OpCode changes size
        let op_type: OperationType = OperationType::from_op_code(&op_code_view[0])?;

        let frame_size: usize = match op_type.fixed_size() {
            None => {
                if self.used_space() < 3 {
                    return Ok(None);
                }
                let length_view: RingBufferView<u8> = self.peek(3)?;
                u16::from_be_bytes([length_view[1], length_view[2]]) as usize
            }
            Some(size) => size,
        };

        Ok(Some(Head {
            op_type,
            length: frame_size,
        }))
    }

    fn pop_frame_data(&mut self, head: &Head) -> Result<Vec<u8>, AppError> {
        let view: RingBufferView<u8> = self.pop(head.length)?;
        let frame_vec: Vec<u8> = view.into();
        Ok(frame_vec)
    }
}
