use crate::{calls, types::*, utils, Internal, Result};
use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};
use getset::CopyGetters;
use nix::sys::time::TimeValLike;
use std::{
    os::unix::io::RawFd,
    sync::{Mutex, MutexGuard},
};

/// Direction types
pub trait Direction {
    //const TYPES: &'static [BufferType];
    fn buffer_type(&self) -> BufferType;
}

macro_rules! direction_impl {
    ($($(#[$($meta:meta)*])* $type:ident {
        $($(#[$($variant_meta:meta)*])* $content_type:ident = $buffer_type:ident,)*
    })*) => {
        $(
            enum_impl! {
                $(#[$($meta)*])*
                enum $type {
                    $(
                        $(#[$($variant_meta)*])*
                        $content_type = BufferType::$buffer_type as u32,
                    )*
                }
            }

            impl From<$type> for BufferType {
                fn from(type_: $type) -> Self {
                    unsafe { core::mem::transmute(type_) }
                }
            }

            impl Direction for $type {
                //const TYPES: &'static [BufferType] = &[$(BufferType::$buf_types),*];
                fn buffer_type(&self) -> BufferType {
                    (*self).into()
                }
            }
        )*
    };
}

/*
enum_impl! {
    /// Buffer content type
    enum ContentType {
        Video,
        Vbi,
        SlicedVbi,
        VideoOverlay,
        VideoMplane,
        Sdr,
        Meta,
    }
}
*/

direction_impl! {
    /// Capture (input direction)
    In {
        /// Video capture
        Video = VideoCapture,
        Vbi = VbiCapture,
        SlicedVbi = SlicedVbiCapture,
        VideoOverlay = VideoOverlay,
        VideoMplane = VideoCaptureMplane,
        Sdr = SdrCapture,
        Meta = MetaCapture,
    }

    /// Render (output direction)
    Out  {
        Video = VideoOutput,
        Vbi = VbiOutput,
        SlicedVbi = SlicedVbiOutput,
        VideoOverlay = VideoOutputOverlay,
        VideoMplane = VideoOutputMplane,
        Sdr = SdrOutput,
        Meta = MetaOutput,
    }
}

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

/// Something which can be used as timestamp
pub trait IsTimestamp {
    /// Convert from timeval
    fn from_timestamp(timeval: TimeVal) -> Self;

    /// Convert into timeval
    fn into_timestamp(self) -> TimeVal;
}

impl IsTimestamp for TimeVal {
    fn from_timestamp(timeval: TimeVal) -> Self {
        timeval
    }

    fn into_timestamp(self) -> TimeVal {
        self
    }
}

impl IsTimestamp for core::time::Duration {
    fn from_timestamp(timeval: TimeVal) -> Self {
        core::time::Duration::from_micros(timeval.num_microseconds() as _)
    }

    fn into_timestamp(self) -> TimeVal {
        TimeVal::microseconds(self.as_micros() as _)
    }
}

impl IsTimestamp for std::time::SystemTime {
    fn from_timestamp(timeval: TimeVal) -> Self {
        std::time::SystemTime::UNIX_EPOCH
            + core::time::Duration::from_micros(timeval.num_microseconds() as _)
    }

    fn into_timestamp(self) -> TimeVal {
        TimeVal::microseconds(
            self.duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_micros() as _,
        )
    }
}

impl Buffer {
    /// Get timestamp
    pub fn timestamp<T: IsTimestamp>(&self) -> T {
        T::from_timestamp(self.timestamp)
    }

    /// Set timestamp
    pub fn set_timestamp<T: IsTimestamp>(&mut self, time: T) {
        self.timestamp = time.into_timestamp();
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
pub trait Method {
    /// Corresponding memory type
    const MEMORY: Memory;

    /// Initialize pointer to data
    fn init(buffer: &Buffer, fd: RawFd) -> Result<*mut u8>;

    /// Deinitialize pointer to data
    fn done(buffer: &Buffer, pointer: *mut u8);

    /// Update buffer before enqueueing
    fn update(_buffer: &mut Buffer, _pointer: *mut u8) {}
}

/// Memory mapping
#[derive(Debug, Clone, Copy)]
pub struct Mmap;

impl Method for Mmap {
    const MEMORY: Memory = Memory::Mmap;

    fn init(buffer: &Buffer, fd: RawFd) -> Result<*mut u8> {
        use nix::sys::mman::{mmap, MapFlags, ProtFlags};

        unsafe_call!(mmap(
            core::ptr::null_mut(),
            buffer.length as _,
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

    fn init(buffer: &Buffer, _fd: RawFd) -> Result<*mut u8> {
        let mut buffer = Vec::<u8>::with_capacity(buffer.length as _);

        let pointer = buffer.as_mut_ptr();

        let _ = ManuallyDrop::new(buffer);

        Ok(pointer)
    }

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
        Met::init(&buffer, fd)?;

        let data = Self {
            pointer: core::ptr::null_mut(),
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

    fn try_enqueue(&mut self, fd: RawFd) -> Result<()> {
        if !self.buffer.is_queued() {
            self.enqueue(fd)?;
        }
        Ok(())
    }

    fn dequeue(&mut self, buffer: Internal<Buffer>) {
        self.buffer = buffer;
    }

    fn mark_dequeued(&mut self) {
        self.buffer.mark_dequeued()
    }
}

impl<Met: Method> Drop for BufferState<Met> {
    fn drop(&mut self) {
        Met::done(&mut self.buffer, self.pointer)
    }
}

#[derive(CopyGetters)]
pub struct QueueData<Dir, Met: Method> {
    buffers: Vec<Mutex<BufferState<Met>>>,

    /// Buffers type
    #[getset(get_copy = "pub")]
    type_: Internal<BufferType>,

    _phantom: PhantomData<Dir>,
}

impl<Dir, Met: Method> QueueData<Dir, Met> {
    /// Get actual number of buffers
    pub fn len(&self) -> usize {
        self.buffers.len()
    }
}

impl<Dir, Met: Method> Internal<QueueData<Dir, Met>> {
    /// Create buffers queue
    pub fn new(fd: RawFd, type_: Dir, count: u32) -> Result<Self>
    where
        Dir: Direction,
    {
        let type_ = type_.buffer_type();

        let request_buffers = Internal::<RequestBuffers>::request(fd, type_, Met::MEMORY, count)?;

        let count = request_buffers.count;

        let mut buffers = Vec::with_capacity(count as _);

        for index in 0..count {
            let mut buffer = Internal::<Buffer>::new(type_, Met::MEMORY, index);
            buffer.query(fd)?;
            let data = BufferState::new(fd, buffer)?;
            buffers.push(Mutex::new(data));
        }

        let type_ = type_.into();

        Ok(QueueData {
            buffers,
            type_,
            _phantom: PhantomData,
        }
        .into())
    }

    /// Delete buffers queue
    pub fn del(&mut self, fd: RawFd) -> Result<()> {
        let _request_buffers =
            Internal::<RequestBuffers>::request(fd, *self.type_, Met::MEMORY, 0)?;

        Ok(())
    }

    /// Enqueue all dequeued buffers and start stream
    pub fn start(&self, fd: RawFd) -> Result<()> {
        self.enqueue_all(fd)?;
        self.type_.stream_on(fd)
    }

    /// Stop stream and dequeue all enqueued buffers
    pub fn stop(&self, fd: RawFd) -> Result<()> {
        self.type_.stream_off(fd)?;
        // stream_off removes all buffers from both queues
        // and unlocks all buffers as a side effect

        for data in &self.buffers {
            if let Ok(mut data) = data.try_lock() {
                data.mark_dequeued();
            }
        }

        Ok(())
    }

    /// Enqueue all dequeued buffers
    pub fn enqueue_all(&self, fd: RawFd) -> Result<()> {
        for data in &self.buffers {
            if let Ok(mut data) = data.try_lock() {
                data.try_enqueue(fd)?;
            }
        }
        Ok(())
    }

    /// Try dequeue buffer
    pub fn dequeue(&self, fd: RawFd) -> Result<BufferData<'_, Dir, Met>> {
        let mut buffer = Internal::<Buffer>::new(*self.type_, Met::MEMORY, 0);

        buffer.dequeue(fd)?;

        let index = buffer.index as usize;

        if let Ok(mut data) = self.buffers[index].try_lock() {
            data.dequeue(buffer);
            Ok(BufferData::new(data))
        } else {
            unreachable!();
        }
    }
}

pub struct BufferData<'r, Dir, Met: Method> {
    data: MutexGuard<'r, BufferState<Met>>,
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
    #[inline(always)]
    fn new(data: MutexGuard<'r, BufferState<Met>>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Get curren length of buffer in bytes
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
