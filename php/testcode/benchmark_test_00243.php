<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00243(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $strong);
    if (!$strong) {
        throw new RuntimeException('hardware RNG unavailable');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
