<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00603(BenchmarkRequest $req): BenchmarkResponse {
    $apiUrl = getenv('UPSTREAM_API_URL');
    $content = file_get_contents($apiUrl . '/health');
    return BenchmarkResponse::ok($content);
}
