pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let shift: u32 = req.param("shift").parse().unwrap_or(0);
    match 1u32.checked_shl(shift) {
        Some(v) => super::shared::BenchmarkResponse::ok(&format!("Shifted: {}", v)),
        None => super::shared::BenchmarkResponse::bad_request("Shift amount too large"),
    }
}
