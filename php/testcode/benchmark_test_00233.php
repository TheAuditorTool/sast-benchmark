<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00233(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('sha256', $pass);
    return BenchmarkResponse::ok($hash);
}
