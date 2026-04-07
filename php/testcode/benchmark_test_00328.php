<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00328(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $isStrong);
    if ($isStrong !== true) {
        return BenchmarkResponse::error('not strong');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
