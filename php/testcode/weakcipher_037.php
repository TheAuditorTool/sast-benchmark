<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium_secretbox
function weakcipher037(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_secretbox_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
    $enc = sodium_crypto_secretbox($req->param('data'), $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_sodium_secretbox
    return BenchmarkResponse::ok(base64_encode($nonce . $enc));
}
// vuln-code-snippet end php_weakcipher_sodium_secretbox
