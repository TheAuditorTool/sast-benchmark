<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00606(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $req->param('url'));
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, true);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
