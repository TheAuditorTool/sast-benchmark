<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00835(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, $iv);
    return BenchmarkResponse::ok($enc);
}
