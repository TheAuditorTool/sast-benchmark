<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01043(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->header('Origin');
    $allowed = ['https://app.example.com', 'https://admin.example.com'];
    if (in_array($origin, $allowed, true)) {
        header('Access-Control-Allow-Origin: ' . $origin);
    }
    return BenchmarkResponse::ok('cors handled');
}
