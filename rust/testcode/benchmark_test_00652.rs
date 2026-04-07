pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    let hashed = sha256_hex(&user_input);
    eprintln!("[INFO] input_hash={}", hashed);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn sha256_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}
