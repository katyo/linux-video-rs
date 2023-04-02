use crate::{
    calls,
    safe_ref::{Lock, Mut, Ref},
    types::*,
    ContentType, Direction, DirectionImpl, In, Internal, IsTimestamp, MethodImpl, Out, Result,
};
use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    num::NonZeroUsize,
};
use getset::CopyGetters;
use std::{
    collections::VecDeque,
    os::unix::io::RawFd,
    sync::atomic::{AtomicBool, Ordering},
};

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
    /// Get timestamp
    pub fn timestamp<T: IsTimestamp>(&self) -> T {
        T::from_time_val(self.timestamp)
    }

    /// Set timestamp
    pub fn set_timestamp<T: IsTimestamp>(&mut self, time: T) {
        self.timestamp = time.into_time_val();
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

    /// Set time code
    pub fn set_timecode(&mut self, timecode: Option<TimeCode>) {
        if let Some(timecode) = timecode {
            self.timecode = timecode;
            self.flags |= BufferFlag::TimeCode;
        } else {
            self.timecode = unsafe { MaybeUninit::zeroed().assume_init() };
            self.flags &= !BufferFlag::TimeCode;
        }
    }

    /// Is buffer locked by driver
    pub fn is_queued(&self) -> bool {
        self.flags.contains(BufferFlag::Queued)
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
    pub fn query(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::query_buf(fd, self.as_mut()).map(|_| ()))
    }

    /// Queue buffer
    pub fn queue(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::q_buf(fd, self.as_mut()).map(|_| ()))
    }

    /// Dequeue buffer
    pub fn dequeue(&mut self, fd: RawFd) -> Result<()> {
        unsafe_call!(calls::dq_buf(fd, self.as_mut()).map(|_| ()))
    }

    pub fn mark_dequeued(&mut self) {
        self.flags &= !BufferFlag::Queued;
    }
}

/// I/O method types
pub trait Method: MethodImpl {
    /// Corresponding memory type
    const MEMORY: Memory;
}

/// Memory mapping
#[derive(Debug, Clone, Copy)]
pub struct Mmap;

impl Method for Mmap {
    const MEMORY: Memory = Memory::Mmap;
}

impl MethodImpl for Mmap {
    fn init(buffer: &Buffer, fd: RawFd) -> Result<*mut u8> {
        use nix::sys::mman::{mmap, MapFlags, ProtFlags};

        unsafe_call!(mmap(
            None,
            NonZeroUsize::new(buffer.length as _).unwrap(),
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd,
            buffer.m.offset as _,
        ))
        .map(|pointer| pointer as _)
    }

    fn done(buffer: &Buffer, pointer: *mut u8) {
        use nix::sys::mman::munmap;

        let _ = unsafe_call!(munmap(pointer as *mut _, buffer.length as _));
    }
}

/// Userspace pointer
#[derive(Debug, Clone, Copy)]
pub struct UserPtr;

impl Method for UserPtr {
    const MEMORY: Memory = Memory::UserPtr;
}

impl MethodImpl for UserPtr {
    fn init(buffer: &Buffer, _fd: RawFd) -> Result<*mut u8> {
        let mut buffer = Vec::<u8>::with_capacity(buffer.length as _);

        let pointer = buffer.as_mut_ptr();

        let _ = ManuallyDrop::new(buffer);

        Ok(pointer)
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn done(buffer: &Buffer, pointer: *mut u8) {
        let _ = unsafe { Vec::<u8>::from_raw_parts(pointer, 0, buffer.length as _) };
    }

    fn update(buffer: &mut Buffer, pointer: *mut u8) {
        buffer.m.userptr = pointer as _;
    }
}

struct BufferState<Met: Method> {
    pointer: *mut u8,
    buffer: Internal<Buffer>,
    _phantom: PhantomData<Met>,
}

unsafe impl<Met: Method> Send for BufferState<Met> {}

impl<Met: Method> core::ops::Deref for BufferState<Met> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<Met: Method> core::ops::DerefMut for BufferState<Met> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl<Met: Method> BufferState<Met> {
    fn new(fd: RawFd, buffer: Internal<Buffer>) -> Result<Self> {
        let pointer = Met::init(&buffer, fd)?;

        let data = Self {
            pointer,
            buffer,
            _phantom: PhantomData,
        };

        Ok(data)
    }

    fn enqueue(&mut self, fd: RawFd) -> Result<()> {
        // update buffer data
        Met::update(&mut self.buffer, self.pointer);
        // add buffer to queue
        self.buffer.queue(fd)
    }

    fn reuse(&mut self, buffer: Internal<Buffer>) {
        self.buffer = buffer;
    }

    fn mark_dequeued(&mut self) {
        self.buffer.mark_dequeued()
    }
}

impl<Met: Method> Drop for BufferState<Met> {
    fn drop(&mut self) {
        Met::done(&self.buffer, self.pointer)
    }
}

#[derive(CopyGetters)]
pub struct QueueData<Dir, Met: Method> {
    /// Requested buffers
    buffers: Vec<Ref<Mut<BufferState<Met>>>>,

    /// Dequeued buffers indexes
    dequeued: Mut<VecDeque<u32>>,

    /// Stream on flag
    on: AtomicBool,

    /// Buffers type
    #[getset(get_copy = "pub")]
    buffer_type: Internal<BufferType>,

    _phantom: PhantomData<Dir>,
}

impl<Dir, Met: Method> QueueData<Dir, Met> {
    /// Queue is empty
    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    /// Get actual number of buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }
}

impl<Dir, Met: Method> Internal<QueueData<Dir, Met>> {
    /// Create buffers queue
    pub fn new(fd: RawFd, content_type: ContentType, count: u32) -> Result<Self>
    where
        Dir: Direction,
    {
        let buffer_type = Dir::buffer_type(content_type);

        let request_buffers =
            Internal::<RequestBuffers>::request(fd, buffer_type, Met::MEMORY, count)?;

        let count = request_buffers.count;

        let mut buffers = Vec::with_capacity(count as _);

        for index in 0..count {
            let mut buffer = Internal::<Buffer>::new(buffer_type, Met::MEMORY, index);
            buffer.query(fd)?;
            let data = BufferState::new(fd, buffer)?;

            buffers.push(Ref::new(Mut::new(data)));
        }

        Ok(QueueData {
            buffers,
            dequeued: Mut::new(VecDeque::with_capacity(count as _)),
            on: AtomicBool::new(false),
            buffer_type: buffer_type.into(),
            _phantom: PhantomData,
        }
        .into())
    }

    /// Delete buffers queue
    pub fn del(&mut self, fd: RawFd) -> Result<()> {
        self.off(fd)?;

        let _request_buffers =
            Internal::<RequestBuffers>::request(fd, *self.buffer_type, Met::MEMORY, 0)?;

        Ok(())
    }

    /// Is queue started
    #[inline(always)]
    fn is_on(&self) -> bool {
        self.on.load(Ordering::SeqCst)
    }

    /// Start stream
    fn on(&self, fd: RawFd) -> Result<()> {
        let type_ = *self.buffer_type.as_ref() as int;

        unsafe_call!(calls::stream_on(fd, &type_).map(|_| ()))?;

        self.on.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Stop stream and mark all buffers as dequeued
    fn off(&self, fd: RawFd) -> Result<()> {
        let type_ = *self.buffer_type.as_ref() as int;

        unsafe_call!(calls::stream_off(fd, &type_).map(|_| ()))?;

        self.on.store(false, Ordering::SeqCst);

        // stream_off removes all buffers from both queues
        // and unlocks all buffers as a side effect
        self.dequeue_queued();

        Ok(())
    }

    /// Dequeue all buffers
    fn dequeue_all(&self) {
        for index in 0..self.buffers.len() {
            let buffer_ref = &self.buffers[index];
            if Ref::strong_count(buffer_ref) == 1 {
                buffer_ref.lock().mark_dequeued();
                self.dequeued.lock().push_back(index as _);
            }
        }
    }

    /// Dequeue queued buffers
    fn dequeue_queued(&self) {
        for index in 0..self.buffers.len() {
            let buffer_ref = &self.buffers[index];
            if Ref::strong_count(buffer_ref) == 1 {
                let mut buffer_data = buffer_ref.lock();
                if buffer_data.is_queued() {
                    buffer_data.mark_dequeued();
                    self.dequeued.lock().push_back(index as _);
                }
            }
        }
    }

    /// Dequeue single unused buffer
    fn dequeue_unused(&self) -> Option<BufferRef<Dir, Met>> {
        for index in 0..self.buffers.len() {
            let buffer_ref = &self.buffers[index];
            if Ref::strong_count(buffer_ref) == 1 && !buffer_ref.lock().is_queued() {
                self.dequeued.lock().push_back(index as _);
                return Some(BufferRef::new(buffer_ref));
            }
        }
        None
    }

    /// Enqueue ready dequeued buffers
    fn enqueue_ready(&self, fd: RawFd) -> Result<()> {
        // we need enqueue only first N buffers which is ready
        // (already processed by user)
        while let Some(first) = {
            let dequeued = self.dequeued.lock();
            dequeued.front().copied()
        } {
            let buffer_ref = &self.buffers[first as usize];
            if Ref::strong_count(buffer_ref) == 1 {
                let mut buffer_data = buffer_ref.lock();
                buffer_data.enqueue(fd)?;
                self.dequeued.lock().pop_front();
            } else {
                // stop on first not ready buffer to preserve sequence
                break;
            }
        }

        Ok(())
    }

    /// Try dequeue buffer
    fn dequeue(&self, fd: RawFd) -> Result<BufferRef<Dir, Met>> {
        let mut buffer = Internal::<Buffer>::new(*self.buffer_type, Met::MEMORY, 0);
        buffer.dequeue(fd)?;
        let index = buffer.index as usize;
        let buffer_ref = &self.buffers[index];
        if Ref::strong_count(buffer_ref) == 1 {
            buffer_ref.lock().reuse(buffer);
            self.dequeued.lock().push_back(index as _);
            Ok(BufferRef::new(buffer_ref))
        } else {
            unreachable!();
        }
    }

    /// Get next buffer to read or write
    pub fn next(&self, fd: RawFd) -> Result<BufferRef<Dir, Met>>
    where
        Dir: Direction,
    {
        Dir::next(self, fd)
    }
}

impl DirectionImpl for In {
    fn next<Met: Method>(
        queue: &Internal<QueueData<Self, Met>>,
        fd: RawFd,
    ) -> Result<BufferRef<Self, Met>> {
        if queue.is_on() {
            queue.enqueue_ready(fd)?;
        } else {
            queue.dequeue_all();
            queue.enqueue_ready(fd)?;
            queue.on(fd)?;
        }
        queue.dequeue(fd)
    }
}

impl DirectionImpl for Out {
    fn next<Met: Method>(
        queue: &Internal<QueueData<Self, Met>>,
        fd: RawFd,
    ) -> Result<BufferRef<Self, Met>> {
        queue.enqueue_ready(fd)?;
        if queue.is_on() {
            queue.dequeue(fd)
        } else {
            if let Some(buffer) = queue.dequeue_unused() {
                return Ok(buffer);
            }
            queue.on(fd)?;
            queue.dequeue(fd)
        }
    }
}

pub struct BufferRef<Dir, Met: Method> {
    data: Ref<Mut<BufferState<Met>>>,
    _phantom: PhantomData<Dir>,
}

impl<Dir, Met: Method> BufferRef<Dir, Met> {
    #[inline(always)]
    fn new(data: &Ref<Mut<BufferState<Met>>>) -> Self {
        Self {
            data: data.clone(),
            _phantom: PhantomData,
        }
    }

    /// Get access to buffer data
    pub fn lock(&self) -> BufferData<'_, Dir, Met> {
        BufferData {
            data: self.data.lock(),
            _phantom: PhantomData,
        }
    }

    /// Try get access to buffer data
    pub fn try_lock(&self) -> Option<BufferData<'_, Dir, Met>> {
        Some(BufferData {
            data: self.data.try_lock()?,
            _phantom: PhantomData,
        })
    }
}

impl<Dir, Met: Method> core::fmt::Display for BufferRef<Dir, Met> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.data.lock().buffer.as_ref().fmt(f)
    }
}

pub struct BufferData<'r, Dir, Met: Method> {
    data: Lock<'r, BufferState<Met>>,
    _phantom: PhantomData<Dir>,
}

impl<'r, Dir, Met: Method> core::ops::Deref for BufferData<'r, Dir, Met> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.data.buffer
    }
}

impl<'r, Met: Method> core::ops::DerefMut for BufferData<'r, Out, Met> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data.buffer
    }
}

impl<'r, Dir, Met: Method> AsRef<[u8]> for BufferData<'r, Dir, Met> {
    fn as_ref(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.pointer, self.len()) }
    }
}

impl<'r, Met: Method> AsMut<[u8]> for BufferData<'r, Out, Met> {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.data.pointer, self.len()) }
    }
}

impl<'r, Met: Method> BufferData<'r, Out, Met> {
    /// Set new size of buffer
    ///
    /// New size should be less than or equal to capacity.
    /// If new size greater than capacity it will be set to be equal to capacity.
    pub fn set_len(&mut self, len: usize) {
        self.data.buffer.bytes_used = self.data.buffer.length.min(len as _);
    }
}

impl<'r, Dir, Met: Method> BufferData<'r, Dir, Met> {
    /// Check no used bytes in buffer
    pub fn is_empty(&self) -> bool {
        self.data.buffer.bytes_used == 0
    }

    /// Get used data of buffer in bytes
    pub fn len(&self) -> usize {
        self.data.buffer.bytes_used as _
    }

    /// Get available buffer capacity in bytes
    pub fn capacity(&self) -> usize {
        self.data.buffer.length as _
    }
}

impl<'r, Dir, Met: Method> core::fmt::Display for BufferData<'r, Dir, Met> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.data.buffer.as_ref().fmt(f)
    }
}
