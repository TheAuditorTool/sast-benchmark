<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00863(BenchmarkRequest $req): BenchmarkResponse {
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', getenv('KEY'), 0, $iv);
    return BenchmarkResponse::ok($enc);
}
