//! CWE-327: RC4 stream cipher with biased keystream.

// vuln-code-snippet start testcodeCrypto004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = b"rc4-secret-key";

    let mut state: Vec<u8> = (0..=255).collect();
    let mut j: u8 = 0;
    for i in 0..256 {
        j = j.wrapping_add(state[i]).wrapping_add(key[i % key.len()]); // vuln-code-snippet target-line testcodeCrypto004
        state.swap(i, j as usize);
    }
    // Simulates: rc4::Rc4::new(key).apply_keystream(&mut data)

    let output: Vec<u8> = plaintext.bytes().enumerate().map(|(i, b)| b ^ state[i % 256]).collect();
    super::shared::BenchmarkResponse::ok(&format!("RC4 encrypted: {:x?}", &output[..]))
}
// vuln-code-snippet end testcodeCrypto004
