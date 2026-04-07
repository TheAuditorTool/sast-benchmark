<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00261(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_ARGON2ID);
    return BenchmarkResponse::ok($hash);
}
