<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01124(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->param('pass');
    $key = md5($password);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, random_bytes(16));
    return BenchmarkResponse::ok($enc);
}
