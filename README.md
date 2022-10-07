# Linux V4L2 API for Rust

This crates intended to provide access to Linux V4L2 APIs without any limitations.

The primary design goal is an optimal balance between safety and overhead.
The implementation much closer to system calls than v4l.
Interface types wraps kernel types to avoid unnecessary copying.

At end this is my demure attempt does things right.
