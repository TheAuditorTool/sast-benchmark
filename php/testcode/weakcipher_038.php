<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium_xchacha20
function weakcipher038(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_aead_xchacha20poly1305_ietf_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_AEAD_XCHACHA20POLY1305_IETF_NPUBBYTES);
    $enc = sodium_crypto_aead_xchacha20poly1305_ietf_encrypt($req->param('data'), '', $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_sodium_xchacha20
    return BenchmarkResponse::ok(base64_encode($nonce . $enc));
}
// vuln-code-snippet end php_weakcipher_sodium_xchacha20
