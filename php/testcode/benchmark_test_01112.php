<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01112(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('HMAC_SECRET');
    $mac = hash_hmac('sha256', $data, $key);
    return BenchmarkResponse::json(['mac' => $mac]);
}
