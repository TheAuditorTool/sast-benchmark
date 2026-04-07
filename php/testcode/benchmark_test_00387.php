<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00387(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('HMAC_KEY');
    $mac = hash_hmac('sha512', $data, $key);
    return BenchmarkResponse::json(['mac' => $mac]);
}
