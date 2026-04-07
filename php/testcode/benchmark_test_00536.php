<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00536(BenchmarkRequest $req): BenchmarkResponse {
    $ip = $req->param('ip');
    if (!filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'http://' . $ip . '/api');
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
