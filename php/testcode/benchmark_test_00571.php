<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00571(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->bodyStr();
    $key = getenv('MAC_SECRET');
    $mac = hash_hmac('sha256', $data, $key);
    return BenchmarkResponse::ok($mac);
}
