<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00524(BenchmarkRequest $req): BenchmarkResponse {
    $fp = fsockopen('127.0.0.1', 6379, $errno, $errstr, 2);
    if ($fp) {
        $key = 'user:' . (int)$req->param('id');
        fwrite($fp, "GET $key\r\n");
        $response = fgets($fp);
        fclose($fp);
        return BenchmarkResponse::ok($response);
    }
    return BenchmarkResponse::error('Redis unavailable');
}
