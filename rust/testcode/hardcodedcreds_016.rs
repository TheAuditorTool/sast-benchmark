//! CWE-798: Encrypted data envelope decrypted via KMS service call at runtime.

// vuln-code-snippet start testcodeHardcodedcreds016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let ciphertext_blob = b"encrypted_key_material_from_config";
    // Simulates: kms_client.decrypt(DecryptRequest { ciphertext_blob })
    let plaintext_key = kms_decrypt(ciphertext_blob); // vuln-code-snippet target-line testcodeHardcodedcreds016
    let result = format!("Action {} with KMS-decrypted key", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn kms_decrypt(ciphertext: &[u8]) -> Vec<u8> {
    // Simulates: aws_sdk_kms::Client::decrypt()
    ciphertext.to_vec()
}
// vuln-code-snippet end testcodeHardcodedcreds016
