<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00434(BenchmarkRequest $req): BenchmarkResponse {
    $token = base64_encode(random_bytes(32));
    return BenchmarkResponse::ok($token);
}
