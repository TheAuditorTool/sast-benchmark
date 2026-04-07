<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00136(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(16));
    return BenchmarkResponse::ok($token);
}
