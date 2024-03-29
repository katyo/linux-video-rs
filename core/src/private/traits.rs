use crate::{
    impls::{BufferRef, QueueData},
    Buffer, Internal, Method, Result,
};
use std::os::unix::io::RawFd;

/// Direction implementation details
pub trait DirectionImpl: Sized {
    /// Get next frame buffer from queue
    fn next<Met: Method>(
        queue: &Internal<QueueData<Self, Met>>,
        fd: RawFd,
    ) -> Result<BufferRef<Self, Met>>;
}

/// I/O method implementation details
pub trait MethodImpl {
    /// Initialize pointer to data
    fn init(buffer: &Buffer, fd: RawFd) -> Result<*mut u8>;

    /// Deinitialize pointer to data
    fn done(buffer: &Buffer, pointer: *mut u8);

    /// Update buffer before enqueueing
    fn update(_buffer: &mut Buffer, _pointer: *mut u8) {}
}
