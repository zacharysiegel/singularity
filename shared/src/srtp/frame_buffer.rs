use crate::error::{AppError, AppErrorStatic};
use crate::srtp::protocol::{Frame, Head, OpCode, OperationType};
use crate::srtp::ring_buffer::{RingBuffer, RingBufferView};

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

            if head.frame_length() > bytes_remaining {
                break;
            }

            let frame_data: Vec<u8> = self.pop_frame_data(&head)?;
            let frame: Frame = Frame::try_from(frame_data.as_slice())?;
            frames.push(frame);
        }
        Ok(frames)
    }

    fn peek_frame_head(&self) -> Result<Option<Head>, AppError> {
        if self.used_space() < size_of::<OpCode>() {
            return Ok(None);
        }
        let op_code_view: RingBufferView<u8> = self.peek(size_of::<OpCode>())?;
        let op_type: OperationType = OperationType::from_op_code(op_code_view[0])?; // Must be modified if OpCode changes size

        let data_length: usize = match op_type.fixed_size() {
            Some(size) => size,
            None => {
                if self.used_space() < op_type.head_length() {
                    return Ok(None);
                }
                let length_view: RingBufferView<u8> = self.peek(op_type.head_length())?;
                u32::from_be_bytes([length_view[1], length_view[2], length_view[3], length_view[4]]) as usize // Must be modified if OpCode or Head.data_length changes size
            }
        };

        Ok(Some(Head {
            op_type,
            data_length,
        }))
    }

    fn pop_frame_data(&mut self, head: &Head) -> Result<Vec<u8>, AppError> {
        let view: RingBufferView<u8> = self.pop(head.head_length() + head.data_length)?;
        let frame_vec: Vec<u8> = view.into();
        Ok(frame_vec)
    }
}
