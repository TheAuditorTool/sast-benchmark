<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00596(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $data = $req->post('data');
    $mac = hash_hmac('sha256', $data, $key);
    return BenchmarkResponse::json(['mac' => $mac, 'key_id' => bin2hex(substr($key, 0, 4))]);
}
