<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00550(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_stream_xchacha20_keygen();
    $nonce = random_bytes(SODIUM_CRYPTO_STREAM_XCHACHA20_NONCEBYTES);
    $stream = sodium_crypto_stream_xchacha20(strlen($req->param('data')), $nonce, $key);
    return BenchmarkResponse::ok(base64_encode($stream));
}
