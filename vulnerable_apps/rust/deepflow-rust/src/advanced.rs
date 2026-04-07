//! Advanced Rust patterns for SAST extraction testing.
//!
//! This module demonstrates patterns that challenge AST extractors:
//! - Inline modules
//! - Macro definitions and invocations
//! - Extern blocks (FFI)
//! - Lifetime parameters
//! - Const generics
//! - Unsafe traits
//! - Const functions

use crate::sinks;
use std::marker::PhantomData;

// ============================================================================
// INLINE MODULES - Nested module declarations
// ============================================================================

/// Inline module with nested items
pub mod inner {
    use super::sinks;

    /// Nested function in inline module
    pub fn process_in_module(input: &str) -> Result<String, String> {
        let result = format!("INNER[{}]", input);
        // TAINT SINK: Data flows into nested module
        sinks::write_to_file("/tmp/inner_module.txt", &result)
    }

    /// Deeply nested module
    pub mod deep {
        use super::super::sinks;

        pub fn deep_process(input: &str) -> Result<String, String> {
            let result = format!("DEEP[{}]", input);
            // TAINT SINK: Deep module sink
            sinks::execute_shell(&format!("echo {}", result))
        }

        /// Even deeper nesting
        pub mod deeper {
            use super::super::super::sinks;

            pub fn deepest_process(input: &str) -> Result<String, String> {
                let result = format!("DEEPEST[{}]", input);
                // TAINT SINK: Deepest module sink
                sinks::log_data("DEEPEST", &result);
                Ok(result)
            }
        }
    }

    /// Private module
    mod private {
        pub fn secret_process(input: &str) -> String {
            format!("SECRET[{}]", input)
        }
    }

    /// Uses private module internally
    pub fn use_private(input: &str) -> String {
        private::secret_process(input)
    }
}

// ============================================================================
// MACRO DEFINITIONS - macro_rules! patterns
// ============================================================================

/// Simple transformation macro
#[macro_export]
macro_rules! transform {
    ($input:expr) => {
        format!("TRANSFORMED[{}]", $input)
    };
    ($input:expr, $prefix:expr) => {
        format!("{}[{}]", $prefix, $input)
    };
}

/// Macro that generates a sink call
#[macro_export]
macro_rules! sink_to_file {
    ($path:expr, $data:expr) => {
        crate::sinks::write_to_file($path, $data)
    };
}

/// Macro that generates a function
#[macro_export]
macro_rules! make_processor {
    ($name:ident, $transform:expr) => {
        pub fn $name(input: &str) -> String {
            let transformed: fn(&str) -> String = $transform;
            transformed(input)
        }
    };
}

// Generate processors using macro
make_processor!(upper_processor, |s: &str| s.to_uppercase());
make_processor!(lower_processor, |s: &str| s.to_lowercase());
make_processor!(reverse_processor, |s: &str| s.chars().rev().collect());

/// Variadic-style macro for chaining
#[macro_export]
macro_rules! chain_transforms {
    ($input:expr) => { $input.to_string() };
    ($input:expr, $transform:expr) => {
        $transform($input)
    };
    ($input:expr, $transform:expr, $($rest:expr),+) => {
        chain_transforms!($transform($input), $($rest),+)
    };
}

/// Macro with different arm patterns
#[macro_export]
macro_rules! process_variant {
    // Match literal pattern
    (upper $input:expr) => {
        $input.to_uppercase()
    };
    // Match with guard-like pattern
    (lower $input:expr) => {
        $input.to_lowercase()
    };
    // Match identifier pattern
    (reverse $input:expr) => {
        $input.chars().rev().collect::<String>()
    };
    // Match expression with block
    (custom $input:expr => $body:block) => {
        {
            let input = $input;
            $body
        }
    };
    // Default arm
    ($input:expr) => {
        $input.to_string()
    };
}

/// Recursive macro for building structures
#[macro_export]
macro_rules! build_pipeline {
    // Base case
    () => { Vec::new() };
    // Single stage
    ($stage:expr) => {
        vec![$stage.to_string()]
    };
    // Multiple stages
    ($stage:expr, $($rest:expr),+) => {{
        let mut stages = vec![$stage.to_string()];
        stages.extend(build_pipeline!($($rest),+));
        stages
    }};
}

/// Use macros with tainted data
pub fn macro_pipeline(input: String) -> Result<String, String> {
    // TAINT FLOW: input -> macro expansion -> sink
    let transformed = transform!(input);
    let prefixed = transform!(transformed, "PREFIX");

    // TAINT SINK: Macro result to file
    sink_to_file!("/tmp/macro_output.txt", &prefixed)
}

/// Chain macros with tainted data
pub fn macro_chain(input: String) -> Result<String, String> {
    let result = chain_transforms!(
        &input,
        |s: &str| s.to_uppercase(),
        |s: String| format!("[{}]", s)
    );

    // TAINT SINK: Chained macro result
    sinks::execute_query(&format!("INSERT INTO macros VALUES ('{}')", result))
}

// ============================================================================
// EXTERN BLOCKS - FFI declarations
// ============================================================================

// External C functions (FFI)
// Note: C runtime functions are automatically linked on most platforms
extern "C" {
    // Standard C library function
    fn strlen(s: *const i8) -> usize;

    // System call wrapper
    fn system(command: *const i8) -> i32;

    // Memory allocation
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);

    // Environment access
    fn getenv(name: *const i8) -> *const i8;

    // Variadic function (TheAuditor extracts is_variadic)
    fn printf(format: *const i8, ...) -> i32;
}

// Extern block with system ABI (stdcall on win32, C elsewhere)
#[cfg(windows)]
extern "system" {
    fn GetLastError() -> u32;
}

/// Rust functions callable from C
#[no_mangle]
pub extern "C" fn process_from_c(input: *const i8, len: usize) -> *mut i8 {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        // TAINT SOURCE: FFI input
        let slice = std::slice::from_raw_parts(input as *const u8, len);
        let input_str = std::str::from_utf8_unchecked(slice);

        // TAINT FLOW: FFI data -> processing
        let result = format!("FFI[{}]", input_str);

        // Allocate and return (caller must free)
        let ptr = malloc(result.len() + 1);
        if !ptr.is_null() {
            std::ptr::copy_nonoverlapping(result.as_ptr(), ptr, result.len());
            *ptr.add(result.len()) = 0;
        }
        ptr as *mut i8
    }
}

/// Use FFI with tainted data
pub fn ffi_strlen(input: &str) -> usize {
    // SAFETY: CString::as_ptr returns a valid null-terminated C string.
    // strlen only reads until null terminator, which CString guarantees.
    unsafe {
        // TAINT FLOW: Rust string -> C string -> FFI
        let c_string = std::ffi::CString::new(input).unwrap_or_default();
        strlen(c_string.as_ptr())
    }
}

/// DANGEROUS: Execute command via FFI
pub fn ffi_system(command: &str) -> i32 {
    // SAFETY: CString::as_ptr returns a valid null-terminated C string.
    // WARNING: User input must be sanitized before passing to this function.
    unsafe {
        // TAINT SINK: Command execution via FFI
        let c_string = std::ffi::CString::new(command).unwrap_or_default();
        system(c_string.as_ptr())
    }
}

/// Get environment variable via FFI
pub fn ffi_getenv(name: &str) -> Option<String> {
    unsafe {
        // TAINT SOURCE: Environment variable
        let c_name = std::ffi::CString::new(name).unwrap_or_default();
        let result = getenv(c_name.as_ptr());
        if result.is_null() {
            None
        } else {
            let c_str = std::ffi::CStr::from_ptr(result);
            Some(c_str.to_string_lossy().into_owned())
        }
    }
}

// ============================================================================
// LIFETIME PARAMETERS - Complex lifetime annotations
// ============================================================================

/// Struct with lifetime parameter
#[derive(Debug)]
pub struct BorrowedData<'a> {
    pub data: &'a str,
    pub metadata: Option<&'a str>,
}

impl<'a> BorrowedData<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            metadata: None,
        }
    }

    pub fn with_metadata(data: &'a str, metadata: &'a str) -> Self {
        Self {
            data,
            metadata: Some(metadata),
        }
    }

    /// Method returning borrowed data
    pub fn get_data(&self) -> &'a str {
        self.data
    }

    /// Method with lifetime elision
    pub fn process(&self) -> String {
        format!("BORROWED[{}]", self.data)
    }
}

/// Multiple lifetime parameters
pub struct MultiLifetime<'a, 'b>
where
    'b: 'a,
{
    pub short: &'a str,
    pub long: &'b str,
}

impl<'a, 'b> MultiLifetime<'a, 'b>
where
    'b: 'a,
{
    pub fn new(short: &'a str, long: &'b str) -> Self {
        Self { short, long }
    }

    /// Return the shorter-lived reference
    pub fn get_short(&self) -> &'a str {
        self.short
    }

    /// Return the longer-lived reference
    pub fn get_long(&self) -> &'b str {
        self.long
    }
}

/// Static lifetime
pub struct StaticData {
    pub data: &'static str,
}

impl StaticData {
    pub const fn new(data: &'static str) -> Self {
        Self { data }
    }
}

/// Function with explicit lifetime
pub fn process_borrowed<'a>(input: &'a str) -> &'a str {
    // Return a slice of the input (maintains lifetime)
    if input.len() > 5 {
        &input[..5]
    } else {
        input
    }
}

/// Function with multiple lifetime parameters
pub fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,
{
    if x.len() >= y.len() {
        x
    } else {
        // This works because 'b: 'a
        unsafe { std::mem::transmute::<&'b str, &'a str>(y) }
    }
}

/// Lifetime with trait bounds
pub fn process_with_lifetime<'a, T>(data: &'a T) -> String
where
    T: AsRef<str> + 'a,
{
    format!("LIFETIME[{}]", data.as_ref())
}

/// Process borrowed data to sink
pub fn borrowed_to_sink(data: BorrowedData<'_>) -> Result<String, String> {
    let result = data.process();
    // TAINT SINK: Borrowed data flows to sink
    sinks::write_to_file("/tmp/borrowed.txt", &result)
}

// ============================================================================
// CONST GENERICS - Compile-time constants in types
// ============================================================================

/// Fixed-size buffer with const generic
#[derive(Debug, Clone)]
pub struct FixedBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuffer<N> {
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut buffer = Self::new();
        let bytes = s.as_bytes();
        let copy_len = bytes.len().min(N);
        buffer.data[..copy_len].copy_from_slice(&bytes[..copy_len]);
        buffer.len = copy_len;
        buffer
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data[..self.len]).unwrap_or("")
    }

    pub fn capacity(&self) -> usize {
        N
    }
}

impl<const N: usize> Default for FixedBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Matrix with const generic dimensions
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new() -> Self {
        Self {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < ROWS && col < COLS {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if row < ROWS && col < COLS {
            self.data[row][col] = value;
            true
        } else {
            false
        }
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

/// Function with const generic parameter
pub fn process_fixed<const SIZE: usize>(input: &str) -> FixedBuffer<SIZE> {
    FixedBuffer::from_str(input)
}

/// Const generic with bounds
pub fn process_array<T, const N: usize>(arr: [T; N]) -> [T; N]
where
    T: Clone,
{
    arr
}

/// Process fixed buffer to sink
pub fn fixed_buffer_sink<const N: usize>(buffer: FixedBuffer<N>) -> Result<String, String> {
    let content = buffer.as_str();
    // TAINT SINK: Fixed buffer data flows to sink
    sinks::execute_shell(&format!("echo {}", content))
}

// ============================================================================
// UNSAFE TRAITS - Traits that require unsafe impl
// ============================================================================

/// Unsafe trait for raw pointer operations
pub unsafe trait RawProcessor {
    fn process_raw(&self, ptr: *const u8, len: usize) -> Vec<u8>;
}

/// Trait with supertraits (TheAuditor extracts supertrait relationships)
pub trait NamedProcessor: std::fmt::Debug + Clone {
    fn name(&self) -> &str;
    fn process(&self, input: &str) -> String;
}

/// Trait with multiple supertraits and associated types
pub trait AdvancedProcessor: Send + Sync + 'static {
    type Input;
    type Output;
    type Error;

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

/// Unsafe implementation
unsafe impl RawProcessor for String {
    fn process_raw(&self, ptr: *const u8, len: usize) -> Vec<u8> {
        if ptr.is_null() || len == 0 {
            return Vec::new();
        }
        // SAFETY: Caller guarantees ptr is valid for len bytes and properly aligned.
        // The slice is only used for reading within this scope.
        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            let mut result = Vec::with_capacity(len + self.len());
            result.extend_from_slice(self.as_bytes());
            result.extend_from_slice(slice);
            result
        }
    }
}

/// Marker trait (unsafe to implement)
pub unsafe trait TaintMarker {}

/// Mark types as potentially tainted
unsafe impl TaintMarker for String {}
unsafe impl TaintMarker for Vec<u8> {}
unsafe impl<'a> TaintMarker for &'a str {}

// ============================================================================
// CONST FUNCTIONS - Compile-time evaluation
// ============================================================================

/// Const function for compile-time computation
pub const fn const_hash(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    let mut i = 0;
    while i < bytes.len() {
        hash = hash.wrapping_mul(33).wrapping_add(bytes[i] as u64);
        i += 1;
    }
    hash
}

/// Const function with generics
pub const fn const_max<const A: usize, const B: usize>() -> usize {
    if A > B {
        A
    } else {
        B
    }
}

/// Const function returning array
pub const fn const_init_array<const N: usize>() -> [u8; N] {
    [0u8; N]
}

/// Const struct initialization
pub const DEFAULT_STATIC: StaticData = StaticData::new("default");

// ============================================================================
// PHANTOM DATA - Zero-sized type markers
// ============================================================================

/// Struct using PhantomData for type-level programming
pub struct TypedId<T> {
    id: u64,
    _marker: PhantomData<T>,
}

impl<T> TypedId<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn get(&self) -> u64 {
        self.id
    }
}

/// Phantom lifetime
pub struct PhantomLifetime<'a, T> {
    data: T,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T> PhantomLifetime<'a, T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _lifetime: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.data
    }
}

// ============================================================================
// ATTRIBUTE PATTERNS - Various attribute usages
// ============================================================================

/// Conditional compilation
#[cfg(feature = "advanced")]
pub fn feature_gated_function(input: &str) -> String {
    format!("FEATURE[{}]", input)
}

/// Always compiled version
#[cfg(not(feature = "advanced"))]
pub fn feature_gated_function(input: &str) -> String {
    format!("BASIC[{}]", input)
}

/// Platform-specific code
#[cfg(target_os = "linux")]
pub fn platform_specific() -> &'static str {
    "linux"
}

#[cfg(target_os = "windows")]
pub fn platform_specific() -> &'static str {
    "windows"
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn platform_specific() -> &'static str {
    "other"
}

/// Deprecated function
#[deprecated(since = "1.0.0", note = "Use new_processor instead")]
pub fn old_processor(input: &str) -> String {
    format!("OLD[{}]", input)
}

/// Replacement function
pub fn new_processor(input: &str) -> String {
    format!("NEW[{}]", input)
}

/// Must use result
#[must_use = "Result must be handled"]
pub fn important_operation(input: &str) -> Result<String, String> {
    Ok(format!("IMPORTANT[{}]", input))
}

/// Cold path hint
#[cold]
pub fn error_handler(error: &str) -> String {
    format!("ERROR[{}]", error)
}

/// Inline hint
#[inline]
pub fn fast_transform(input: &str) -> String {
    input.to_uppercase()
}

/// Always inline
#[inline(always)]
pub fn always_inline_transform(input: &str) -> String {
    input.to_lowercase()
}

/// Never inline
#[inline(never)]
pub fn never_inline_transform(input: &str) -> String {
    format!("NOINLINE[{}]", input)
}

/// Repr attributes for layout control
#[repr(C)]
pub struct CLayoutStruct {
    pub a: u32,
    pub b: u64,
    pub c: u32,
}

#[repr(packed)]
pub struct PackedStruct {
    pub a: u8,
    pub b: u32,
    pub c: u8,
}

#[repr(align(16))]
pub struct AlignedStruct {
    pub data: [u8; 32],
}

/// Repr for enums
#[repr(u8)]
pub enum SmallEnum {
    A = 0,
    B = 1,
    C = 2,
}

#[repr(C)]
pub enum CEnum {
    Variant1,
    Variant2 { x: i32, y: i32 },
    Variant3(u64),
}

// ============================================================================
// COMBINED ADVANCED PATTERNS
// ============================================================================

/// Combines multiple advanced features
pub fn advanced_pipeline<'a, const N: usize>(
    input: &'a str,
    buffer: &mut FixedBuffer<N>,
) -> Result<BorrowedData<'a>, String> {
    // Use macro
    let transformed = transform!(input);

    // Use const generic buffer
    *buffer = FixedBuffer::from_str(&transformed);

    // Return with lifetime
    Ok(BorrowedData::new(input))
}

/// Process through all advanced features to sink
pub fn advanced_to_sink(input: String) -> Result<String, String> {
    // Inner module processing
    let inner_result = inner::process_in_module(&input)?;

    // Deep module processing
    let deep_result = inner::deep::deep_process(&input)?;

    // Macro processing
    let macro_result = transform!(input, "MACRO");

    // Lifetime processing
    let borrowed = BorrowedData::new(&macro_result);
    let borrowed_result = borrowed.process();

    // Const generic processing
    let buffer: FixedBuffer<256> = FixedBuffer::from_str(&borrowed_result);

    // Combine all results
    let combined = format!(
        "INNER:{}\nDEEP:{}\nBORROWED:{}\nBUFFER:{}",
        inner_result,
        deep_result,
        borrowed_result,
        buffer.as_str()
    );

    // TAINT SINK: All advanced processing flows to sink
    sinks::write_to_file("/tmp/advanced_combined.txt", &combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_module() {
        let result = inner::use_private("test");
        assert!(result.contains("SECRET"));
    }

    #[test]
    fn test_transform_macro() {
        let result = transform!("test");
        assert!(result.contains("TRANSFORMED"));
    }

    #[test]
    fn test_fixed_buffer() {
        let buffer: FixedBuffer<10> = FixedBuffer::from_str("hello");
        assert_eq!(buffer.as_str(), "hello");
        assert_eq!(buffer.capacity(), 10);
    }

    #[test]
    fn test_borrowed_data() {
        let data = BorrowedData::new("test");
        assert_eq!(data.get_data(), "test");
    }

    #[test]
    fn test_const_hash() {
        let hash = const_hash(b"test");
        assert!(hash > 0);
    }
}
