<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01116(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = json_decode($body, true, 512, JSON_THROW_ON_ERROR);
    return BenchmarkResponse::json($data);
}
