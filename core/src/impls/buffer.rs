use crate::{
    calls,
    safe_ref::{Lock, Mut},
    types::*,
    DirectionImpl, Internal, IsTimestamp, MethodImpl, Result,
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

/// Direction types
pub trait Direction: DirectionImpl {
    const IN: bool;
    const OUT: bool;

    fn buffer_type(content: ContentType) -> BufferType;
}

macro_rules! direction_impl {
    ($($(#[$($meta:meta)*])* $type:ident ($is_input:literal) {
        $($(#[$($variant_meta:meta)*])* $content_type:ident = $buffer_type:ident,)*
    })*) => {
        $(
            $(#[$($meta)*])*
            pub struct $type;

            impl Direction for $type {
                const IN: bool = $is_input;
                const OUT: bool = !$is_input;

                fn buffer_type(content: ContentType) -> BufferType {
                    match content {
                        $(
                            ContentType::$content_type => BufferType::$buffer_type,
                        )*
                    }
                }
            }
        )*
    };
}

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

impl ContentType {
    /// All content types
    pub const ALL: [Self; 7] = [
        Self::Video,
        Self::Vbi,
        Self::SlicedVbi,
        Self::VideoOverlay,
        Self::VideoMplane,
        Self::Sdr,
        Self::Meta,
    ];

    /// All video types
    pub const VIDEO: [Self; 3] = [Self::Video, Self::VideoOverlay, Self::VideoMplane];

    /// Is video type
    pub fn is_video(self) -> bool {
        matches!(self, Self::Video | Self::VideoOverlay | Self::VideoMplane)
    }

    /// Get buffer type using direction
    pub fn buffer_type<Dir: Direction>(self) -> BufferType {
        Dir::buffer_type(self)
    }
}

direction_impl! {
    /// Capture (input direction)
    In (true) {
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
    Out (false) {
        Video = VideoOutput,
        Vbi = VbiOutput,
        SlicedVbi = SlicedVbiOutput,
        VideoOverlay = VideoOutputOverlay,
        VideoMplane = VideoOutputMplane,
        Sdr = SdrOutput,
        Meta = MetaOutput,
    }
}

impl BufferType {
    /// All buffer types
    pub const ALL: [Self; 14] = [
        Self::VideoCapture,
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::SdrCapture,
        Self::MetaCapture,
        Self::VideoOutput,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
        Self::SdrOutput,
        Self::MetaOutput,
    ];

    /// Capture buffer types
    pub const CAPTURE: [Self; 7] = [
        Self::VideoCapture,
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::SdrCapture,
        Self::MetaCapture,
    ];

    /// Output buffer types
    pub const OUTPUT: [Self; 7] = [
        Self::VideoOutput,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
        Self::SdrOutput,
        Self::MetaOutput,
    ];

    /// Video buffer types
    pub const VIDEO: [Self; 6] = [
        Self::VideoCapture,
        Self::VideoOverlay,
        Self::VideoCaptureMplane,
        Self::VideoOutput,
        Self::VideoOutputOverlay,
        Self::VideoOutputMplane,
    ];

    /// Vbi buffer types
    pub const VBI: [Self; 4] = [
        Self::VbiCapture,
        Self::SlicedVbiCapture,
        Self::VbiOutput,
        Self::SlicedVbiOutput,
    ];

    /// Sdr buffer types
    pub const SDR: [Self; 2] = [Self::SdrCapture, Self::SdrOutput];

    /// Meta buffer types
    pub const META: [Self; 2] = [Self::MetaCapture, Self::MetaOutput];

    /// Single-planar video buffer types
    pub const VIDEO_SPLANE: [Self; 2] = [Self::VideoCapture, Self::VideoOutput];

    /// Multi-planar video buffer types
    pub const VIDEO_MPLANE: [Self; 2] = [Self::VideoCaptureMplane, Self::VideoOutputMplane];

    /// Overlay video buffer types
    pub const VIDEO_OVERLAY: [Self; 2] = [Self::VideoOverlay, Self::VideoOutputOverlay];

    /// Check that buffer type is supported according to capabilities
    pub fn is_supported(self, capabilities: CapabilityFlag) -> bool {
        match self {
            Self::VideoCapture => capabilities.contains(CapabilityFlag::VideoCapture),
            Self::VbiCapture => capabilities.contains(CapabilityFlag::VbiCapture),
            Self::SlicedVbiCapture => capabilities.contains(CapabilityFlag::SlicedVbiCapture),
            Self::VideoOverlay => capabilities.contains(CapabilityFlag::VideoOverlay),
            Self::VideoCaptureMplane => capabilities.contains(CapabilityFlag::VideoCaptureMplane),
            Self::SdrCapture => capabilities.contains(CapabilityFlag::SdrCapture),
            Self::MetaCapture => capabilities.contains(CapabilityFlag::MetaCapture),
            Self::VideoOutput => capabilities.contains(CapabilityFlag::VideoOutput),
            Self::VbiOutput => capabilities.contains(CapabilityFlag::VbiOutput),
            Self::SlicedVbiOutput => capabilities.contains(CapabilityFlag::SlicedVbiOutput),
            Self::VideoOutputOverlay => capabilities.contains(CapabilityFlag::VideoOutputOverlay),
            Self::VideoOutputMplane => capabilities.contains(CapabilityFlag::VideoOutputMplane),
            Self::SdrOutput => capabilities.contains(CapabilityFlag::SdrOutput),
            Self::MetaOutput => capabilities.contains(CapabilityFlag::MetaOutput),
        }
    }

    /// Get corresponding content type
    pub fn content(&self) -> ContentType {
        match self {
            Self::VideoCapture | Self::VideoOutput => ContentType::Video,
            Self::VbiCapture | Self::VbiOutput => ContentType::Vbi,
            Self::SlicedVbiCapture | Self::SlicedVbiOutput => ContentType::SlicedVbi,
            Self::VideoOverlay | Self::VideoOutputOverlay => ContentType::VideoOverlay,
            Self::VideoCaptureMplane | Self::VideoOutputMplane => ContentType::VideoMplane,
            Self::SdrCapture | Self::SdrOutput => ContentType::Sdr,
            Self::MetaCapture | Self::MetaOutput => ContentType::Meta,
        }
    }
}

impl Internal<BufferType> {
    /// Start stream
    pub fn stream_on(self, fd: RawFd) -> Result<()> {
        let type_ = *self.as_ref() as int;

        unsafe_call!(calls::stream_on(fd, &type_).map(|_| ()))
    }

    /// Stop stream
    pub fn stream_off(self, fd: RawFd) -> Result<()> {
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
    buffers: Vec<Mut<BufferState<Met>>>,

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

            buffers.push(Mut::new(data));
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
        self.buffer_type.stream_on(fd)?;

        self.on.store(true, Ordering::SeqCst);

        Ok(())
    }

    /// Stop stream and mark all buffers as dequeued
    fn off(&self, fd: RawFd) -> Result<()> {
        self.buffer_type.stream_off(fd)?;

        self.on.store(false, Ordering::SeqCst);

        // stream_off removes all buffers from both queues
        // and unlocks all buffers as a side effect
        self.dequeue_queued();

        Ok(())
    }

    /// Dequeue all buffers
    fn dequeue_all(&self) {
        for index in 0..self.buffers.len() {
            if let Some(mut data) = {
                let buffer = &self.buffers[index];
                buffer.try_lock()
            } {
                data.mark_dequeued();
                self.dequeued.lock().push_back(index as _);
            }
        }
    }

    /// Dequeue queued buffers
    fn dequeue_queued(&self) {
        for index in 0..self.buffers.len() {
            if let Some(mut data) = {
                let buffer = &self.buffers[index];
                buffer.try_lock()
            } {
                if data.is_queued() {
                    data.mark_dequeued();
                    self.dequeued.lock().push_back(index as _);
                }
            }
        }
    }

    /// Dequeue single unused buffer
    fn dequeue_unused(&self) -> Option<BufferData<'_, Dir, Met>> {
        for index in 0..self.buffers.len() {
            if let Some(data) = {
                let buffer = &self.buffers[index];
                buffer.try_lock()
            } {
                if !data.is_queued() {
                    self.dequeued.lock().push_back(index as _);
                    return Some(BufferData::new(data));
                }
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
            if let Some(mut data) = {
                let buffer = &self.buffers[first as usize];
                buffer.try_lock()
            } {
                data.enqueue(fd)?;
                self.dequeued.lock().pop_front();
            } else {
                // stop on first not ready buffer to preserve sequence
                break;
            }
        }

        Ok(())
    }

    /// Try dequeue buffer
    fn dequeue(&self, fd: RawFd) -> Result<BufferData<'_, Dir, Met>> {
        let mut buffer = Internal::<Buffer>::new(*self.buffer_type, Met::MEMORY, 0);

        buffer.dequeue(fd)?;

        let index = buffer.index as usize;

        if let Some(mut data) = {
            let buffer = &self.buffers[index];
            buffer.try_lock()
        } {
            data.reuse(buffer);
            self.dequeued.lock().push_back(index as _);
            Ok(BufferData::new(data))
        } else {
            unreachable!();
        }
    }

    /// Get next buffer to read or write
    pub fn next(&self, fd: RawFd) -> Result<BufferData<'_, Dir, Met>>
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
    ) -> Result<BufferData<'_, Self, Met>> {
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
    ) -> Result<BufferData<'_, Self, Met>> {
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
    #[inline(always)]
    fn new(data: Lock<'r, BufferState<Met>>) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

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
