<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium
function weakcipher006(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = sodium_crypto_secretbox_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
    $encrypted = sodium_crypto_secretbox($data, $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_sodium
    return BenchmarkResponse::json(['ciphertext' => base64_encode($nonce . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_sodium
