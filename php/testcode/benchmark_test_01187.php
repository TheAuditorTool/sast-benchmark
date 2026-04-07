<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01187(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = 'sk-prod-4f8a2b1c9d3e7f6a';
    $response = file_get_contents('https://api.payments.example.com/charge?key=' . $apiKey);
    return BenchmarkResponse::json(json_decode($response, true));
}
