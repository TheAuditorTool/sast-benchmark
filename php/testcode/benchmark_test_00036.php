<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00036(BenchmarkRequest $req): BenchmarkResponse {
    $token = base64_encode((string)time());
    return BenchmarkResponse::ok($token);
}
