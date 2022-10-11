use crate::{calls, types::*, Internal, Result};
use core::mem::{ManuallyDrop, MaybeUninit};
use getset::CopyGetters;
use std::os::unix::io::RawFd;

impl Internal<BufferType> {
    /// Start stream
    pub fn stream_on(&self, fd: RawFd) -> Result<()> {
        let type_ = *self.as_ref() as int;

        unsafe_call!(calls::stream_on(fd, &type_).map(|_| ()))
    }

    /// Stop stream
    pub fn stream_off(&self, fd: RawFd) -> Result<()> {
        let type_ = *self.as_ref() as int;

        unsafe_call!(calls::stream_off(fd, &type_).map(|_| ()))
    }
}

impl Internal<RequestBuffers> {
    /// Request buffers
    pub fn request(fd: RawFd, type_: BufferType, memory: Memory, count: u32) -> Result<Self> {
        let req_bufs = MaybeUninit::<RequestBuffers>::zeroed();

        unsafe_call!({
            let mut req_bufs = req_bufs.assume_init();
            req_bufs.type_ = type_;
            req_bufs.memory = memory;
            req_bufs.count = count;
            calls::req_bufs(fd, &mut req_bufs).map(|_| req_bufs.into())
        })
    }
}

impl Internal<Buffer> {
    /// Instantiate buffer
    pub fn new(type_: BufferType, memory: Memory, index: u32) -> Self {
        let buffer = MaybeUninit::<Buffer>::zeroed();
        let mut buffer = unsafe { buffer.assume_init() };

        buffer.type_ = type_;
        buffer.memory = memory;
        buffer.index = index;

        buffer.into()
    }

    /// Query bufer by index
    pub fn query(fd: RawFd, type_: BufferType, memory: Memory, index: u32) -> Result<Self> {
        let mut buffer = Self::new(type_, memory, index);

        unsafe_call!(calls::query_buf(fd, buffer.as_mut()).map(|_| buffer))
    }

    /// Queue buffer
    pub fn queue(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::q_buf(fd, self.as_mut()).map(|_| ()))
    }

    /// Dequeue buffer
    pub fn dequeue(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::dq_buf(fd, self.as_mut()).map(|_| ()))
    }
}

struct BufferData {
    pointer: *mut u8,
    length: u32,
}

impl BufferData {
    fn new(fd: RawFd, buffer: &Buffer) -> Result<Self> {
        let pointer = match buffer.memory {
            Memory::Mmap => {
                use nix::sys::mman::{mmap, MapFlags, ProtFlags};

                unsafe {
                    mmap(
                        core::ptr::null_mut(),
                        buffer.length as _,
                        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                        MapFlags::MAP_SHARED,
                        fd,
                        buffer.m.offset as _,
                    )
                    .map(|pointer| pointer as *mut u8)
                }?
            }
            Memory::UserPtr => {
                let mut data = Vec::<u8>::with_capacity(buffer.length as _);
                let pointer = data.as_mut_ptr();
                let _ = ManuallyDrop::new(data);
                pointer
            }
            _ => unimplemented!(),
        };

        Ok(Self {
            pointer,
            length: buffer.length,
        })
    }

    fn set(&self, buffer: &mut Buffer) {
        match buffer.memory {
            Memory::Mmap => {
                // nothing to do
            }
            Memory::UserPtr => {
                buffer.m.userptr = self.pointer as _;
            }
            _ => unimplemented!(),
        }
    }

    fn del(&mut self, memory: Memory) -> Result<()> {
        match memory {
            Memory::Mmap => {
                use nix::sys::mman::munmap;

                unsafe_call!(munmap(self.pointer as *mut _, self.length as _))?;
            }
            Memory::UserPtr => {
                let _ = unsafe { Vec::<u8>::from_raw_parts(self.pointer, 0, self.length as _) };
                // drop buffer
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}

#[derive(CopyGetters)]
pub struct IoBuffers {
    buffers: Vec<BufferData>,

    /// Buffers type
    #[getset(get_copy = "pub")]
    type_: BufferType,

    /// Memory type
    #[getset(get_copy = "pub")]
    memory: Memory,
}

impl IoBuffers {
    /// Get actual number of buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }
}

impl Internal<IoBuffers> {
    /// Create buffer queue
    pub fn new(fd: RawFd, type_: BufferType, memory: Memory, count: u32) -> Result<Self> {
        let request_buffers = Internal::<RequestBuffers>::request(fd, type_, memory, count)?;

        let count = request_buffers.count;

        let mut buffers = Vec::with_capacity(count as _);

        for index in 0..count {
            let mut buffer = Internal::<Buffer>::query(fd, type_, memory, index)?;
            let buffer_data = BufferData::new(fd, &mut buffer)?;

            buffer_data.set(&mut buffer);
            buffer.queue(fd)?;

            buffers.push(buffer_data);
        }

        Ok(IoBuffers {
            buffers,
            type_,
            memory,
        }
        .into())
    }

    pub fn del(&mut self, fd: RawFd) -> Result<()> {
        let type_ = self.type_;
        let memory = self.memory;

        for buffer in &mut self.buffers {
            buffer.del(memory)?;
        }

        let _request_buffers = Internal::<RequestBuffers>::request(fd, type_, memory, 0)?;

        Ok(())
    }

    pub fn dequeue(&self, fd: RawFd) -> Result<IoBuffer<'_>> {
        let mut buffer = Internal::<Buffer>::new(self.type_, self.memory, 0);

        buffer.dequeue(fd)?;

        Ok(IoBuffer {
            buffers: self,
            buffer,
        })
    }
}

pub struct IoBuffer<'b> {
    buffers: &'b Internal<IoBuffers>,
    buffer: Internal<Buffer>,
}

impl<'b> core::ops::Deref for IoBuffer<'b> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<'b> IoBuffer<'b> {
    /// Get curren length of buffer in bytes
    pub fn len(&self) -> usize {
        self.buffer.bytes_used as _
    }

    /// Get available buffer capacity in bytes
    pub fn capacity(&self) -> usize {
        self.buffer.length as _
    }

    /// Set new size of buffer
    ///
    /// New size should be less than or equal to capacity.
    /// If new size greater than capacity it will be set to be equal to capacity.
    pub fn resize(&mut self, size: usize) {
        let size = self.buffer.length.max(size as _);

        self.buffer.bytes_used = size;
    }
}

impl<'b> AsRef<[u8]> for IoBuffer<'b> {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.buffers.buffers[self.buffer.index as usize].pointer,
                self.buffer.bytes_used as _,
            )
        }
    }
}

impl<'b> AsMut<[u8]> for IoBuffer<'b> {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.buffers.buffers[self.buffer.index as usize].pointer,
                self.buffer.bytes_used as _,
            )
        }
    }
}
