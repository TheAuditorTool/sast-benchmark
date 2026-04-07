<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01129(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_BCRYPT, ['cost' => 12]);
    return BenchmarkResponse::ok($hash);
}
