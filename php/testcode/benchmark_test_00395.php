<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00395(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('sha512', $pass);
    return BenchmarkResponse::ok($hash);
}
