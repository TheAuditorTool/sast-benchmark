<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00933(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'gopher://' . $host . ':6379/_INFO');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
