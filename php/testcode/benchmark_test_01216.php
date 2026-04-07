<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01216(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init('https://api.internal.example.com/status');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
