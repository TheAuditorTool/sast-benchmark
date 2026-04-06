<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium_aead
function weakcipher018(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = sodium_crypto_aead_xchacha20poly1305_ietf_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_AEAD_XCHACHA20POLY1305_IETF_NPUBBYTES);
    $ad = 'benchmark-context';
    $encrypted = sodium_crypto_aead_xchacha20poly1305_ietf_encrypt($data, $ad, $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_sodium_aead
    return BenchmarkResponse::json(['ciphertext' => base64_encode($nonce . $encrypted)]);
}
// vuln-code-snippet end php_weakcipher_sodium_aead
