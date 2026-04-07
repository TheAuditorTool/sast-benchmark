<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00895(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_secretbox_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
    $enc = sodium_crypto_secretbox($req->param('data'), $nonce, $key);
    return BenchmarkResponse::ok(base64_encode($nonce . $enc));
}
