<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01214(BenchmarkRequest $req): BenchmarkResponse {
    $endpoint = $req->param('endpoint');
    $ch = curl_init($endpoint);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
