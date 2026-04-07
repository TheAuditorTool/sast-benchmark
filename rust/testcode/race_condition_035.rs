//! CWE-362: Mutable static accessed via unsafe block without any synchronization primitive.

// vuln-code-snippet start testcodeRaceCondition035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let increment: i64 = req.param("value").parse().unwrap_or(1);
    unsafe {
        GLOBAL_COUNTER += increment; // vuln-code-snippet target-line testcodeRaceCondition035
    }
    let val = unsafe { GLOBAL_COUNTER };
    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val))
}

static mut GLOBAL_COUNTER: i64 = 0;
// vuln-code-snippet end testcodeRaceCondition035
