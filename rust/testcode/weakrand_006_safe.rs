//! Weak Random True Negative — CWE-330
//! /dev/urandom read (simulated). Reading from the kernel entropy pool
//! provides cryptographically secure random bytes.

// vuln-code-snippet start testcodeWeakrand006Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    // SAFE: /dev/urandom is a CSPRNG source on Linux
    let token = read_urandom(); // vuln-code-snippet safe-line testcodeWeakrand006Safe

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn read_urandom() -> String {
    // Simulates: std::fs::File::open("/dev/urandom").read(&mut buf)
    let secure_bytes: [u8; 16] = [0xCD; 16]; // placeholder for urandom output
    secure_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
// vuln-code-snippet end testcodeWeakrand006Safe
