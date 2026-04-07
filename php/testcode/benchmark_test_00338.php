<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00338(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_secretstream_xchacha20poly1305_keygen();
    [$state, $header] = sodium_crypto_secretstream_xchacha20poly1305_init_push($key);
    $enc = sodium_crypto_secretstream_xchacha20poly1305_push($state, $req->param('data'));
    return BenchmarkResponse::ok(base64_encode($header . $enc));
}
