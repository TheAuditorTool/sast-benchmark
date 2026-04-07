<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00113(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_DEFAULT);
    return BenchmarkResponse::ok($hash);
}
