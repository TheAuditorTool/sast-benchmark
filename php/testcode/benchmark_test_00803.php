<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00803(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = hash('murmur3a', $data);
    return BenchmarkResponse::ok($hash);
}
