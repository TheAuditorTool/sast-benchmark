<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01022(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'RC4', $key);
    return BenchmarkResponse::ok($enc);
}
