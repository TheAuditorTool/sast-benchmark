<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01059(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(16);
    $staticIv = 'staticiv12345678';
    $enc = openssl_encrypt($req->param('data'), 'BF-CBC', $key, 0, $staticIv);
    return BenchmarkResponse::ok($enc);
}
