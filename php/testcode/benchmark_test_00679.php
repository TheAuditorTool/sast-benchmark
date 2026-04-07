<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00679(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init($req->param('url'));
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, false);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
