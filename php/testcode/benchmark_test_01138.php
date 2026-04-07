<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01138(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $strong);
    if (!$strong) {
        throw new RuntimeException('not strong');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
