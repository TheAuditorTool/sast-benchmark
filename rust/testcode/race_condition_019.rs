//! CWE-362: Inter-thread communication via channel instead of shared mutable state.

// vuln-code-snippet start testcodeRaceCondition019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let message = req.param("message");

    // Simulates: crossbeam::channel::unbounded()
    let result = channel_send(&message); // vuln-code-snippet target-line testcodeRaceCondition019
    super::shared::BenchmarkResponse::ok(&format!("Sent: {}", result))
}

fn channel_send(msg: &str) -> String {
    // Simulates: let (tx, rx) = crossbeam::channel::unbounded(); tx.send(msg);
    format!("channel_msg_{}", msg)
}
// vuln-code-snippet end testcodeRaceCondition019
