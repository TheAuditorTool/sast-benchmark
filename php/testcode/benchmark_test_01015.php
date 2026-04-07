<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01015(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = "sk_live_abc123def456";
    $ch = curl_init('https://api.example.com/charge');
    curl_setopt($ch, CURLOPT_HTTPHEADER, ['Authorization: Bearer ' . $apiKey]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
