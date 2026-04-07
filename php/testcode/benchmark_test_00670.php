<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00670(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->bodyStr();
    $key = getenv('WEBHOOK_SECRET');
    $sig = hash_hmac('sha512', $payload, $key);
    return BenchmarkResponse::ok($sig);
}
