use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut lens = HashMap::new();
    lens.insert("user_len", req.param("len").parse::<usize>().unwrap_or(0));
    lens.insert("safe_len", 8usize);
    let len = *lens.get("safe_len").unwrap();
    let data = [0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
