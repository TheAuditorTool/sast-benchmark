<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium_secretstream
function weakcipher048(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_secretstream_xchacha20poly1305_keygen();
    [$state, $header] = sodium_crypto_secretstream_xchacha20poly1305_init_push($key);
    $enc = sodium_crypto_secretstream_xchacha20poly1305_push($state, $req->param('data')); // vuln-code-snippet safe-line php_weakcipher_sodium_secretstream
    return BenchmarkResponse::ok(base64_encode($header . $enc));
}
// vuln-code-snippet end php_weakcipher_sodium_secretstream
