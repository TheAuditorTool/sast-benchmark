<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00729(BenchmarkRequest $req): BenchmarkResponse {
    mt_srand((int)(microtime(true) * 1000));
    $token = mt_rand(100000, 999999);
    return BenchmarkResponse::json(['otp' => $token]);
}
