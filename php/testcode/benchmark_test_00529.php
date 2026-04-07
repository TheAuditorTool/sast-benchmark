<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00529(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('data');
    $checksum = crc32($payload);
    return BenchmarkResponse::json(['data' => $payload, 'signature' => $checksum]);
}
