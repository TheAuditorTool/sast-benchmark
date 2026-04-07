<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00739(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(32);
    $key = hash_pbkdf2('sha256', $pass, $salt, 100000, 32, true);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, 0, random_bytes(12));
    return BenchmarkResponse::ok($enc);
}
