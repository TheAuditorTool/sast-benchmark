<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01074(BenchmarkRequest $req): BenchmarkResponse {
    $key = 'weak_key_123456';
    $iv = random_bytes(8);
    $enc = openssl_encrypt($req->param('data'), 'RC2-CBC', $key, 0, $iv);
    return BenchmarkResponse::ok($enc);
}
