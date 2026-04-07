<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00050(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, true);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
