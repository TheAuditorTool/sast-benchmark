//! Unsafe operations module with raw pointer and memory patterns.

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

/// Buffer allocation with user-controlled size
// vuln-code-snippet start memsafetyBackendBufferAlloc
pub fn dangerous_buffer_alloc(size: usize) -> String {
    if size == 0 {
        return "Zero-size allocation".to_string();
    }

    // UNSAFE: User controlled size
    unsafe {
        let mut vec: Vec<u8> = Vec::with_capacity(size);

        //Setting length without initializing memory
        // This exposes uninitialized memory
        vec.set_len(size); // vuln-code-snippet target-line memsafetyBackendBufferAlloc

        format!("Allocated {} bytes (uninitialized!)", size)
    }
}
// vuln-code-snippet end memsafetyBackendBufferAlloc

/// Raw pointer manipulation with user input
pub fn dangerous_ptr_ops(offset: usize) -> String {
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];

    unsafe {
        let ptr = data.as_ptr();

        // User controlled offset for pointer access
        let value = *ptr.add(offset);

        format!("Value at offset {}: {}", offset, value)
    }
}

/// Use-after-free pattern
pub fn use_after_free_pattern() -> String {
    unsafe {
        let layout = Layout::new::<[u8; 64]>();
        let ptr = alloc(layout);

        if ptr.is_null() {
            return "Allocation failed".to_string();
        }

        // Write some data
        ptr::write_bytes(ptr, 0x41, 64);

        // Free the memory
        dealloc(ptr, layout);

        //Use after free - reading deallocated memory
        let value = *ptr;

        format!("Value after free: {} (UNDEFINED BEHAVIOR!)", value)
    }
}

///Double free pattern
pub fn double_free_pattern() -> String {
    unsafe {
        let layout = Layout::new::<[u8; 32]>();
        let ptr = alloc(layout);

        if ptr.is_null() {
            return "Allocation failed".to_string();
        }

        dealloc(ptr, layout);

        //Double free - freeing already freed memory
        // This is commented out to prevent actual crash, but shows the pattern
        // dealloc(ptr, layout);

        "Double free pattern (simulated)".to_string()
    }
}

///Buffer overflow via unchecked indexing
// vuln-code-snippet start memsafetyBackendBufferOverflow
pub fn buffer_overflow_write(index: usize, value: u8) -> String {
    let mut buffer = [0u8; 16];

    unsafe {
        //No bounds checking on user-controlled index
        let ptr = buffer.as_mut_ptr();
        *ptr.add(index) = value; // vuln-code-snippet target-line memsafetyBackendBufferOverflow
    }

    format!("Wrote {} at index {} (potentially out of bounds!)", value, index)
}
// vuln-code-snippet end memsafetyBackendBufferOverflow

///Transmute with user-controlled data
pub fn dangerous_transmute(bytes: &[u8]) -> String {
    if bytes.len() < 8 {
        return "Not enough bytes for transmute".to_string();
    }

    unsafe {
        //Transmuting user-controlled bytes
        let value: u64 = std::mem::transmute_copy(&bytes[0]);
        format!("Transmuted value: {}", value)
    }
}

///Unsafe deserialization
// vuln-code-snippet start deserBackendDangerousDeserialize
pub fn dangerous_deserialize(payload: &str) -> String {
    // Decode base64
    let decoded = match base64::decode(payload) {
        Ok(d) => d,
        Err(e) => return format!("Decode error: {}", e),
    };

    //Deserializing untrusted data
    match bincode::deserialize::<serde_json::Value>(&decoded) { // vuln-code-snippet target-line deserBackendDangerousDeserialize
        Ok(value) => format!("Deserialized: {:?}", value),
        Err(e) => format!("Deserialize error: {}", e),
    }
}
// vuln-code-snippet end deserBackendDangerousDeserialize

///Unchecked arithmetic
// vuln-code-snippet start intoverflowBackendUncheckedArithmetic
pub fn unchecked_arithmetic(a: i32, b: i32, op: &str) -> i32 {
    //No overflow checking
    match op {
        "add" => a.wrapping_add(b), // vuln-code-snippet target-line intoverflowBackendUncheckedArithmetic
        "mul" => a.wrapping_mul(b),
        "div" => {
            if b == 0 {
                //Division by zero not properly handled in some paths
                0
            } else {
                a / b
            }
        }
        "shl" => a << (b & 31),  // Masking but still potentially wrong
        _ => 0,
    }
}
// vuln-code-snippet end intoverflowBackendUncheckedArithmetic

///Race condition pattern (not thread-safe static)
static mut GLOBAL_COUNTER: i32 = 0;

pub fn racy_increment() -> i32 {
    unsafe {
        //Data race on global mutable static
        GLOBAL_COUNTER += 1;
        GLOBAL_COUNTER
    }
}

///Null pointer dereference pattern
pub fn null_ptr_pattern(should_be_null: bool) -> String {
    let ptr: *const i32 = if should_be_null {
        std::ptr::null()
    } else {
        &42 as *const i32
    };

    unsafe {
        if !ptr.is_null() {
            format!("Value: {}", *ptr)
        } else {
            // Would crash if we dereferenced here
            "Null pointer (not dereferenced)".to_string()
        }
    }
}

// Base64 decoding (simple implementation for demo)
mod base64 {
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        // Simple base64 decode
        let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut output = Vec::new();
        let mut buffer = 0u32;
        let mut bits = 0;

        for byte in input.bytes() {
            if byte == b'=' {
                break;
            }

            let value = alphabet.iter().position(|&b| b == byte)
                .ok_or_else(|| format!("Invalid base64 character: {}", byte as char))?;

            buffer = (buffer << 6) | (value as u32);
            bits += 6;

            if bits >= 8 {
                bits -= 8;
                output.push((buffer >> bits) as u8);
                buffer &= (1 << bits) - 1;
            }
        }

        Ok(output)
    }
}
