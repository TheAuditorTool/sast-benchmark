use std::collections::HashMap;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut vals = HashMap::new();
    vals.insert("user_a", req.param("a").parse::<i32>().unwrap_or(0));
    vals.insert("safe_a", 5i32);
    let a = vals.get("safe_a").unwrap();
    let result = a.checked_mul(2);
    match result {
        Some(r) => super::shared::BenchmarkResponse::ok(&format!("r={}", r)),
        None => super::shared::BenchmarkResponse::bad_request("Overflow"),
    }
}
