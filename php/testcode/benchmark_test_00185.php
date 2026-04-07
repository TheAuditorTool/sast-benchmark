<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00185(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->bodyStr();
    $key = getenv('API_SECRET');
    $sig = hash_hmac('md5', $data, $key);
    return BenchmarkResponse::ok($sig);
}
