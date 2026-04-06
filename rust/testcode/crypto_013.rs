//! CWE-327: User data encrypted with RC4, a stream cipher with known cryptographic weaknesses.

// vuln-code-snippet start testcodeCrypto013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let plaintext = req.param("data");
    let key = b"static-rc4-key";

    let ciphertext = rc4_encrypt(plaintext.as_bytes(), key); // vuln-code-snippet target-line testcodeCrypto013

    super::shared::BenchmarkResponse::ok(&format!("Ciphertext: {:x?}", ciphertext))
}

fn rc4_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // Simulates: rc4::Rc4::new(key.into()).apply_keystream(&mut buf)
    let mut s: Vec<u8> = (0..=255u8).collect();
    let mut j: usize = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }
    let mut i = 0usize;
    let mut j = 0usize;
    data.iter().map(|&b| {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        b ^ s[(s[i] as usize + s[j] as usize) % 256]
    }).collect()
}
// vuln-code-snippet end testcodeCrypto013
