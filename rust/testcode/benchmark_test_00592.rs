pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = b"rc4-secret-key";

    let mut state: Vec<u8> = (0..=255).collect();
    let mut j: u8 = 0;
    for i in 0..256 {
        j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]);
        state.swap(i, j as usize);
    }

    let output: Vec<u8> = plaintext.bytes().enumerate().map(|(i, b)| b ^ state[i % 256]).collect();
    super::shared::BenchmarkResponse::ok(&format!("RC4 encrypted: {:x?}", &output[..]))
}
