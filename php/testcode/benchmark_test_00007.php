<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00007(BenchmarkRequest $req): BenchmarkResponse {
    $key = substr(hash('sha256', getenv('KEY')), 0, 16);
    $enc = openssl_encrypt($req->param('data'), 'AES-128-ECB', $key);
    return BenchmarkResponse::ok($enc);
}
