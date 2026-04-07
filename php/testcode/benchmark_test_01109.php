<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01109(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(32);
    $hash = hash_pbkdf2('sha256', $pass, $salt, 100000);
    return BenchmarkResponse::ok($hash);
}
