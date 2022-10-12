use crate::{calls, types::*, Internal, Result};
use core::mem::{ManuallyDrop, MaybeUninit};
use getset::CopyGetters;
use nix::sys::time::TimeValLike;
use std::{
    os::unix::io::RawFd,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

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

impl Buffer {
    /// Get timestamp as duration
    pub fn duration(&self) -> core::time::Duration {
        core::time::Duration::from_micros(self.timestamp.num_microseconds() as _)
    }

    /// Get timestamp as system time
    pub fn time(&self) -> std::time::SystemTime {
        std::time::SystemTime::UNIX_EPOCH + self.duration()
    }

    /// Buffer has time code
    pub fn has_timecode(&self) -> bool {
        self.flags.contains(BufferFlag::TimeCode)
    }

    /// Buffer time code
    pub fn timecode(&self) -> Option<TimeCode> {
        if self.has_timecode() {
            Some(self.timecode)
        } else {
            None
        }
    }
}

impl core::fmt::Display for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        '#'.fmt(f)?;
        self.sequence.fmt(f)?;
        " @".fmt(f)?;
        self.index.fmt(f)?;
        ' '.fmt(f)?;
        self.timestamp.fmt(f)?;
        if self.has_timecode() {
            ' '.fmt(f)?;
            self.timecode.fmt(f)?;
        }
        ' '.fmt(f)?;
        self.type_.fmt(f)?;
        ' '.fmt(f)?;
        self.memory.fmt(f)?;
        ' '.fmt(f)?;
        self.bytes_used.fmt(f)?;
        '/'.fmt(f)?;
        self.length.fmt(f)?;
        if !self.flags.is_none() {
            ' '.fmt(f)?;
            self.flags.fmt(f)?;
        }
        if self.field != Field::None {
            ' '.fmt(f)?;
            self.field.fmt(f)?;
        }
        Ok(())
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
    buffer: Internal<Buffer>,
    used: AtomicU32,
    queued: bool,
}

impl BufferData {
    #[inline(always)]
    fn used(&self) -> u32 {
        self.used.load(Ordering::SeqCst)
    }

    #[inline(always)]
    fn set_used(&self, used: u32) {
        self.used.store(used, Ordering::SeqCst);
    }

    #[inline(always)]
    fn mark_queued(&mut self) {
        self.queued = true;
    }

    #[inline(always)]
    fn mark_dequeued(&mut self) {
        self.queued = false;
    }

    fn new(fd: RawFd, buffer: Internal<Buffer>) -> Result<Self> {
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
            buffer,
            used: AtomicU32::new(buffer.bytes_used),
            queued: false,
        })
    }

    fn try_queue(&mut self, fd: RawFd) -> Result<bool> {
        Ok(if self.queued {
            false
        } else {
            // update buffer data
            match self.buffer.memory {
                Memory::Mmap => {
                    // nothing to do
                }
                Memory::UserPtr => {
                    self.buffer.m.userptr = self.pointer as _;
                }
                _ => unimplemented!(),
            }

            // update used bytes
            self.buffer.bytes_used = self.used();

            // add buffer to queue
            self.buffer.queue(fd)?;

            self.mark_queued();
            true
        })
    }

    #[inline(always)]
    fn dequeue(&mut self, buffer: Internal<Buffer>) {
        self.set_used(buffer.bytes_used);
        self.buffer = buffer;
        self.mark_dequeued();
    }
}

impl Drop for BufferData {
    fn drop(&mut self) {
        match self.buffer.memory {
            Memory::Mmap => {
                use nix::sys::mman::munmap;

                let _ = unsafe_call!(munmap(self.pointer as *mut _, self.buffer.length as _));
            }
            Memory::UserPtr => {
                let _ =
                    unsafe { Vec::<u8>::from_raw_parts(self.pointer, 0, self.buffer.length as _) };
                // drop buffer
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(CopyGetters)]
pub struct IoQueue {
    buffers: Vec<Arc<BufferData>>,

    /// Buffers type
    #[getset(get_copy = "pub")]
    type_: Internal<BufferType>,

    /// Memory type
    #[getset(get_copy = "pub")]
    memory: Memory,
}

impl IoQueue {
    /// Get actual number of buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }
}

impl Internal<IoQueue> {
    /// Create buffer queue
    pub fn new(fd: RawFd, type_: BufferType, memory: Memory, count: u32) -> Result<Self> {
        let request_buffers = Internal::<RequestBuffers>::request(fd, type_, memory, count)?;

        let count = request_buffers.count;

        let mut buffers = Vec::with_capacity(count as _);

        for index in 0..count {
            let buffer = Internal::<Buffer>::query(fd, type_, memory, index)?;
            let data = Arc::new(BufferData::new(fd, buffer)?);
            buffers.push(data);
        }

        let type_ = type_.into();

        Ok(IoQueue {
            buffers,
            type_,
            memory,
        }
        .into())
    }

    pub fn del(&mut self, fd: RawFd) -> Result<()> {
        let _request_buffers =
            Internal::<RequestBuffers>::request(fd, *self.type_, self.memory, 0)?;

        Ok(())
    }

    pub fn start(&mut self, fd: RawFd) -> Result<()> {
        self.queue(fd)?;
        self.type_.stream_on(fd)
    }

    pub fn stop(&mut self, fd: RawFd) -> Result<()> {
        self.type_.stream_off(fd)?;
        // stream_off removes all buffers from both queues
        // and unlocks all buffers as a side effect

        for data in &mut self.buffers {
            if let Some(data) = Arc::get_mut(data) {
                data.mark_dequeued();
            }
        }

        Ok(())
    }

    pub fn queue(&mut self, fd: RawFd) -> Result<()> {
        for data in &mut self.buffers {
            if let Some(data) = Arc::get_mut(data) {
                data.try_queue(fd)?;
            }
        }
        Ok(())
    }

    pub fn dequeue(&mut self, fd: RawFd) -> Result<IoBuffer> {
        let mut buffer = Internal::<Buffer>::new(*self.type_, self.memory, 0);

        buffer.dequeue(fd)?;

        let data = &mut self.buffers[buffer.index as usize];

        assert!(data.queued);

        Arc::get_mut(data).unwrap().dequeue(buffer);

        let data = data.clone();

        Ok(IoBuffer { data })
    }
}

pub struct IoBuffer {
    data: Arc<BufferData>,
}

impl core::ops::Deref for IoBuffer {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.data.buffer
    }
}

impl IoBuffer {
    /// Get curren length of buffer in bytes
    pub fn len(&self) -> usize {
        self.data.used() as _
    }

    /// Get available buffer capacity in bytes
    pub fn capacity(&self) -> usize {
        self.data.buffer.length as _
    }

    /// Set new size of buffer
    ///
    /// New size should be less than or equal to capacity.
    /// If new size greater than capacity it will be set to be equal to capacity.
    pub fn resize(&mut self, size: usize) {
        let size = self.data.buffer.length.max(size as _);

        self.data.set_used(size);
    }
}

impl AsRef<[u8]> for IoBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.pointer, self.data.used() as _) }
    }
}

impl AsMut<[u8]> for IoBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data.pointer, self.data.used() as _) }
    }
}

impl core::fmt::Display for IoBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.data.buffer.as_ref().fmt(f)
    }
}
