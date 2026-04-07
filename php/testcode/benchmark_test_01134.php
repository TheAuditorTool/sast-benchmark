<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01134(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $nonce = 'fixed_nonce_1234';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, 0, $nonce);
    return BenchmarkResponse::ok($enc);
}
