<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00145(BenchmarkRequest $req): BenchmarkResponse {
    $fp = fopen('/dev/urandom', 'rb');
    $bytes = fread($fp, 32);
    fclose($fp);
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
