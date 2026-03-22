//! Unsafe operations demonstrating memory safety sinks.
//!
//! These operations bypass Rust's safety guarantees and are potential
//! code injection vectors when combined with tainted data.

use std::ptr;

/// TAINT SINK: std::mem::transmute with user data
/// Critical: Can cause undefined behavior with wrong data
// vuln-code-snippet start memsafetyTransmute
pub fn dangerous_transmute(data: &[u8]) -> Result<String, &'static str> {
    if data.len() < 8 {
        return Err("Data too short for transmute");
    }

    // TAINT SINK: Transmuting user-controlled bytes (DANGEROUS!)
    unsafe {
        // SAFETY: THIS IS NOT SAFE - demonstrating vulnerability
        let ptr = data.as_ptr() as *const u64;
        let value: u64 = std::mem::transmute(*ptr); // vuln-code-snippet target-line memsafetyTransmute
        Ok(format!("Transmuted value: {}", value))
    }
}
// vuln-code-snippet end memsafetyTransmute

/// TAINT SINK: Raw pointer write with user data
// vuln-code-snippet start memsafetyWriteToBuffer
pub fn write_to_buffer(buffer: &mut [u8], data: &[u8], offset: usize) -> Result<(), &'static str> {
    if offset + data.len() > buffer.len() { // vuln-code-snippet target-line memsafetyWriteToBuffer
        return Err("Buffer overflow prevented");
    }

    unsafe {
        // TAINT SINK: ptr::write with user-controlled data
        // SAFETY: Bounds checked above, but data content is tainted
        let dest = buffer.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
    }

    Ok(())
}
// vuln-code-snippet end memsafetyWriteToBuffer

/// TAINT SINK: Raw pointer read at user-controlled offset
// vuln-code-snippet start memsafetyReadFromOffset
pub fn read_from_offset(buffer: &[u8], offset: usize) -> Result<u8, &'static str> {
    if offset >= buffer.len() { // vuln-code-snippet target-line memsafetyReadFromOffset
        return Err("Out of bounds read prevented");
    }

    unsafe {
        // TAINT SINK: ptr::read with user-controlled offset
        // SAFETY: Bounds checked, but demonstrates pattern
        let ptr = buffer.as_ptr().add(offset);
        Ok(ptr::read(ptr))
    }
}
// vuln-code-snippet end memsafetyReadFromOffset

/// TAINT SINK: Writing to arbitrary memory location
/// This is extremely dangerous with user-controlled addresses
// vuln-code-snippet start memsafetyWriteToAddress
pub fn write_to_address(address: usize, value: u8) {
    unsafe {
        // TAINT SINK: std::ptr::write to user-controlled address
        // SAFETY: NONE - this is demonstrating a vulnerability
        let ptr = address as *mut u8;
        ptr::write(ptr, value); // vuln-code-snippet target-line memsafetyWriteToAddress
    }
}
// vuln-code-snippet end memsafetyWriteToAddress

/// TAINT SINK: Reading from arbitrary memory location
pub fn read_from_address(address: usize) -> u8 {
    unsafe {
        // TAINT SINK: std::ptr::read from user-controlled address
        // SAFETY: NONE - this is demonstrating a vulnerability
        let ptr = address as *const u8;
        ptr::read(ptr)
    }
}

/// TAINT SINK: Volatile write (for hardware/DMA scenarios)
pub fn volatile_write(address: usize, value: u8) {
    unsafe {
        // TAINT SINK: ptr::write_volatile
        // SAFETY: NONE - demonstrating vulnerability
        let ptr = address as *mut u8;
        ptr::write_volatile(ptr, value);
    }
}

/// TAINT SINK: Volatile read
pub fn volatile_read(address: usize) -> u8 {
    unsafe {
        // TAINT SINK: ptr::read_volatile
        // SAFETY: NONE - demonstrating vulnerability
        let ptr = address as *const u8;
        ptr::read_volatile(ptr)
    }
}

/// TAINT SINK: Copy memory with user-controlled parameters
// vuln-code-snippet start memsafetyCopyMemory
pub fn copy_memory(src: &[u8], dst: &mut [u8], len: usize) -> Result<(), &'static str> {
    if len > src.len() || len > dst.len() { // vuln-code-snippet target-line memsafetyCopyMemory
        return Err("Length exceeds buffer size");
    }

    unsafe {
        // TAINT SINK: ptr::copy_nonoverlapping with user-controlled length
        // SAFETY: Bounds checked above
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), len);
    }

    Ok(())
}
// vuln-code-snippet end memsafetyCopyMemory

/// Unsafe block without SAFETY comment (should be flagged by auditor)
pub fn unsafe_without_comment(data: &mut [u8]) {
    // Missing SAFETY comment - this should be flagged!
    unsafe {
        let ptr = data.as_mut_ptr();
        *ptr = 0xFF;
    }
}

/// Unsafe block with proper SAFETY comment
pub fn unsafe_with_comment(data: &mut [u8]) {
    // SAFETY: We have mutable access to data, and we're only modifying
    // the first byte which is guaranteed to exist due to slice borrow
    unsafe {
        if !data.is_empty() {
            let ptr = data.as_mut_ptr();
            *ptr = 0xFF;
        }
    }
}

/// Multiple unsafe blocks in one function
pub fn multiple_unsafe_blocks(a: &mut [u8], b: &mut [u8]) {
    // First unsafe block - no safety comment
    unsafe {
        if !a.is_empty() {
            let ptr = a.as_mut_ptr();
            *ptr = 0xAA;
        }
    }

    // Some safe code in between
    let _sum = a.len() + b.len();

    // SAFETY: b is borrowed mutably and non-empty check ensures valid access
    unsafe {
        if !b.is_empty() {
            let ptr = b.as_mut_ptr();
            *ptr = 0xBB;
        }
    }
}

/// Raw pointer arithmetic
pub fn pointer_arithmetic(base: *const u8, offset: isize) -> *const u8 {
    unsafe {
        // TAINT SINK: Pointer arithmetic with user-controlled offset
        // SAFETY: NONE - offset could go out of bounds
        base.offset(offset)
    }
}

/// Unchecked array access
// vuln-code-snippet start memsafetyUncheckedAccess
pub fn unchecked_access(data: &[u8], index: usize) -> u8 {
    unsafe {
        // TAINT SINK: Unchecked array access with user index
        // SAFETY: NONE - index is not bounds-checked
        *data.get_unchecked(index) // vuln-code-snippet target-line memsafetyUncheckedAccess
    }
}
// vuln-code-snippet end memsafetyUncheckedAccess

/// Mutable unchecked access
pub fn unchecked_mut_access(data: &mut [u8], index: usize, value: u8) {
    unsafe {
        // TAINT SINK: Unchecked mutable access
        // SAFETY: NONE - index is not bounds-checked
        *data.get_unchecked_mut(index) = value;
    }
}

/// Creating a slice from raw parts
// vuln-code-snippet start memsafetySliceFromRawParts
pub fn slice_from_raw_parts(ptr: *const u8, len: usize) -> &'static [u8] {
    unsafe {
        // TAINT SINK: Creating slice with user-controlled length
        // SAFETY: NONE - ptr and len could be invalid
        std::slice::from_raw_parts(ptr, len) // vuln-code-snippet target-line memsafetySliceFromRawParts
    }
}
// vuln-code-snippet end memsafetySliceFromRawParts

/// Mutable slice from raw parts
pub fn slice_from_raw_parts_mut(ptr: *mut u8, len: usize) -> &'static mut [u8] {
    unsafe {
        // TAINT SINK: Creating mutable slice with user-controlled params
        // SAFETY: NONE - ptr and len could be invalid
        std::slice::from_raw_parts_mut(ptr, len)
    }
}

/// Cast between types unsafely
pub fn unsafe_cast<T, U>(value: T) -> U {
    unsafe {
        // TAINT SINK: Type transmutation
        // SAFETY: NONE - T and U might not have compatible layouts
        std::mem::transmute_copy(&value)
    }
}

/// Drop value without running destructor
pub fn forget_value<T>(value: T) {
    // This is safe but can lead to resource leaks
    std::mem::forget(value);
}

/// Zero out memory (used in crypto, but can be misused)
// vuln-code-snippet start memsafetySecureZero
pub fn secure_zero(data: &mut [u8]) {
    unsafe {
        // SAFETY: data is a valid mutable slice
        ptr::write_bytes(data.as_mut_ptr(), 0, data.len()); // vuln-code-snippet target-line memsafetySecureZero
    }
    // Prevent optimizer from eliding the write
    std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
}
// vuln-code-snippet end memsafetySecureZero
