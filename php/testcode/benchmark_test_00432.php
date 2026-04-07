<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00432(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $salt = random_bytes(32);
    $hash = hash('sha256', $data . $salt);
    return BenchmarkResponse::ok($hash);
}
