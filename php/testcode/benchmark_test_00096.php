<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00096(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = hash('sha3-256', $data);
    return BenchmarkResponse::ok($hash);
}
