<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00080(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, "https://api.internal.com/status");
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
