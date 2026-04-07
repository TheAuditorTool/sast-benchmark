<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_kms_decrypt_call
function hardcodedcreds037(BenchmarkRequest $req): BenchmarkResponse {
    $kms = new stdClass();
    $result = $kms->decrypt(['CiphertextBlob' => base64_decode(getenv('ENCRYPTED_KEY'))]); // vuln-code-snippet safe-line php_hardcoded_kms_decrypt_call
    $key = $result['Plaintext'];
    $data = $req->param('data');
    $iv = random_bytes(16);
    $ciphertext = openssl_encrypt($data, 'AES-256-CBC', $key, 0, $iv);
    return BenchmarkResponse::ok(base64_encode($iv) . ':' . $ciphertext);
}
// vuln-code-snippet end php_hardcoded_kms_decrypt_call
