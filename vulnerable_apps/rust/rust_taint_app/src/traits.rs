//! Trait definitions demonstrating unsafe traits and implementations.
//!
//! This module shows patterns for unsafe trait implementations that
//! TheAuditor's graph strategies should capture.

use std::marker::PhantomData;

/// A regular trait for comparison
pub trait SafeProcessor {
    fn process(&self, data: &[u8]) -> Vec<u8>;
    fn validate(&self, input: &str) -> bool;
}

/// An unsafe trait that requires implementors to uphold invariants
/// UNSAFE TRAIT: Implementors must ensure thread safety
pub unsafe trait UnsafeProcessor {
    /// Process data in an unsafe manner
    /// # Safety
    /// Implementors must ensure the returned pointer is valid for the lifetime 'a
    unsafe fn process_unsafe<'a>(&self, data: *const u8, len: usize) -> *mut u8;

    /// Get a raw pointer to internal state
    /// # Safety
    /// The returned pointer is only valid while self is alive
    unsafe fn get_internal_ptr(&self) -> *const u8;
}

/// Another unsafe trait for memory operations
pub unsafe trait RawMemoryAccess {
    /// Read from arbitrary address
    /// # Safety
    /// Caller must ensure address is valid and readable
    unsafe fn read_at(&self, address: usize) -> u8;

    /// Write to arbitrary address
    /// # Safety
    /// Caller must ensure address is valid and writable
    unsafe fn write_at(&mut self, address: usize, value: u8);
}

/// A data container that implements unsafe traits
pub struct DataContainer {
    data: Vec<u8>,
    capacity: usize,
}

impl DataContainer {
    pub fn new(initial_data: Vec<u8>) -> Self {
        let capacity = initial_data.capacity();
        Self {
            data: initial_data,
            capacity,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl SafeProcessor for DataContainer {
    fn process(&self, data: &[u8]) -> Vec<u8> {
        // Standard implementation
        data.iter().map(|b| b.wrapping_add(1)).collect()
    }

    fn validate(&self, input: &str) -> bool {
        // Standard validation
        !input.is_empty() && input.len() < 1000
    }
}

/// UNSAFE IMPL: Implementing unsafe trait for DataContainer
unsafe impl UnsafeProcessor for DataContainer {
    unsafe fn process_unsafe<'a>(&self, data: *const u8, len: usize) -> *mut u8 {
        // SAFETY: Caller guarantees data pointer is valid for len bytes
        // We allocate new memory for the result
        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            let byte = *data.add(i);
            result.push(byte.wrapping_add(1));
        }

        let ptr = result.as_mut_ptr();
        std::mem::forget(result); // Caller is responsible for freeing
        ptr
    }

    unsafe fn get_internal_ptr(&self) -> *const u8 {
        // SAFETY: Pointer is valid while self is alive
        self.data.as_ptr()
    }
}

/// UNSAFE IMPL: Implementing RawMemoryAccess - very dangerous!
unsafe impl RawMemoryAccess for DataContainer {
    unsafe fn read_at(&self, address: usize) -> u8 {
        // SAFETY: Caller must ensure address is within bounds
        // This is dangerous with user-controlled addresses!
        *(address as *const u8)
    }

    unsafe fn write_at(&mut self, address: usize, value: u8) {
        // SAFETY: Caller must ensure address is valid and writable
        // This is extremely dangerous with user-controlled addresses!
        *(address as *mut u8) = value;
    }
}

/// Unsafe auto trait (marker trait)
pub unsafe auto trait UnsafeMarker {}

/// Negative impl to exclude certain types
impl !UnsafeMarker for String {}
impl !UnsafeMarker for Vec<u8> {}

/// A generic unsafe container
pub struct UnsafeContainer<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

impl<T> UnsafeContainer<T> {
    /// Create a new unsafe container
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        Self {
            ptr: Box::into_raw(boxed),
            _marker: PhantomData,
        }
    }

    /// Get the contained value unsafely
    /// # Safety
    /// Container must not be dropped
    pub unsafe fn get(&self) -> &T {
        &*self.ptr
    }

    /// Get mutable reference unsafely
    /// # Safety
    /// Container must not be dropped and no other references exist
    pub unsafe fn get_mut(&mut self) -> &mut T {
        &mut *self.ptr
    }
}

impl<T> Drop for UnsafeContainer<T> {
    fn drop(&mut self) {
        unsafe {
            // SAFETY: ptr was created from Box::into_raw
            let _ = Box::from_raw(self.ptr);
        }
    }
}

/// UNSAFE IMPL: Send for UnsafeContainer
/// This requires T is Send
unsafe impl<T: Send> Send for UnsafeContainer<T> {}

/// UNSAFE IMPL: Sync for UnsafeContainer
/// This requires T is Sync
unsafe impl<T: Sync> Sync for UnsafeContainer<T> {}

/// A trait for FFI callbacks
pub trait FfiCallback {
    /// Called from C code
    extern "C" fn callback(data: *mut std::ffi::c_void, len: usize);
}

/// A processor that uses FFI
pub struct FfiProcessor {
    callback: Option<extern "C" fn(*mut std::ffi::c_void, usize)>,
}

impl FfiProcessor {
    pub fn new() -> Self {
        Self { callback: None }
    }

    pub fn set_callback(&mut self, cb: extern "C" fn(*mut std::ffi::c_void, usize)) {
        self.callback = Some(cb);
    }

    /// Invoke the callback with user data
    /// TAINT SINK: User data passed to FFI callback
    pub unsafe fn invoke_callback(&self, data: &mut [u8]) {
        if let Some(cb) = self.callback {
            // SAFETY: Callback was set by user, data is valid
            cb(data.as_mut_ptr() as *mut std::ffi::c_void, data.len());
        }
    }
}

impl Default for FfiProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Example of unsafe function declaration
/// # Safety
/// Caller must ensure ptr points to valid memory
pub unsafe fn process_raw_pointer(ptr: *mut u8, len: usize) -> Vec<u8> {
    // SAFETY: Caller guarantees ptr is valid for len bytes
    let slice = std::slice::from_raw_parts(ptr, len);
    slice.to_vec()
}

/// Extern function for FFI
#[no_mangle]
pub extern "C" fn exported_function(data: *const u8, len: usize) -> i32 {
    if data.is_null() || len == 0 {
        return -1;
    }

    unsafe {
        // SAFETY: Caller guarantees data is valid for len bytes
        let slice = std::slice::from_raw_parts(data, len);
        slice.len() as i32
    }
}

/// Variadic extern function (for C interop)
#[cfg(feature = "variadic")]
extern "C" {
    fn variadic_c_function(count: i32, ...) -> i32;
}

/// Extern block declaring C functions
extern "C" {
    /// External C function
    fn external_c_function(data: *const u8, len: usize) -> i32;

    /// Another external function
    fn process_external_data(input: *mut u8, output: *mut u8, len: usize);
}

/// Wrapper that calls external function
pub fn call_external(data: &[u8]) -> i32 {
    unsafe {
        // SAFETY: data slice is valid
        external_c_function(data.as_ptr(), data.len())
    }
}
