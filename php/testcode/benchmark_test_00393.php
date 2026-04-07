<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00393(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $token = crc32($data);
    return BenchmarkResponse::json(['security_token' => $token]);
}
