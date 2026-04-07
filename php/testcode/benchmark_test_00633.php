<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00633(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = sha1($pass . 'static_salt_123');
    return BenchmarkResponse::ok($hash);
}
