<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00578(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('payload');
    $decompressed = gzuncompress(base64_decode($payload));
    $obj = unserialize($decompressed);
    return BenchmarkResponse::ok('processed');
}
