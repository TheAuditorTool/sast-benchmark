<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01190(BenchmarkRequest $req): BenchmarkResponse {
    $apiKey = getenv('PAYMENT_API_KEY');
    $response = file_get_contents('https://api.payments.example.com/charge?key=' . $apiKey);
    return BenchmarkResponse::json(json_decode($response, true));
}
