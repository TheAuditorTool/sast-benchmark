<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_sodium_xchacha20_stream
function weakcipher042(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_stream_xchacha20_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_STREAM_XCHACHA20_NONCEBYTES);
    $stream = sodium_crypto_stream_xchacha20(strlen($req->param('data')), $nonce, $key); // vuln-code-snippet safe-line php_weakcipher_sodium_xchacha20_stream
    return BenchmarkResponse::ok(base64_encode($stream));
}
// vuln-code-snippet end php_weakcipher_sodium_xchacha20_stream
