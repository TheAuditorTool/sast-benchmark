<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00512(BenchmarkRequest $req): BenchmarkResponse {
    $key = substr(hash('sha256', 'static'), 0, 8);
    $iv = random_bytes(8);
    $enc = openssl_encrypt($req->param('data'), 'DES-CBC', $key, 0, $iv);
    return BenchmarkResponse::ok($enc);
}
