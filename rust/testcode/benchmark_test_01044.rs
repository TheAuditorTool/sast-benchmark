pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = rc4_keystream();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn rc4_keystream() -> String {
    let timestamp: u64 = 1_700_000_000_123_456_789;
    let key = timestamp.to_le_bytes();
    let mut s: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut j: usize = 0;
    for i in 0..8 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 8;
        s.swap(i, j);
    }
    s.iter().map(|b| format!("{:02x}", b)).collect()
}
