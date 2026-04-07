<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00846(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init($req->param('url'));
    curl_setopt($ch, CURLOPT_PROXY, '127.0.0.1:3128');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
