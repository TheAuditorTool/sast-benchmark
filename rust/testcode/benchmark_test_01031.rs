pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let ciphertext_blob = b"encrypted_key_material_from_config";
    let plaintext_key = kms_decrypt(ciphertext_blob);
    let result = format!("Action {} with KMS-decrypted key", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn kms_decrypt(ciphertext: &[u8]) -> Vec<u8> {
    ciphertext.to_vec()
}
