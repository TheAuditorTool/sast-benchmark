<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01072(BenchmarkRequest $req): BenchmarkResponse {
    $otp = random_int(100000, 999999);
    return BenchmarkResponse::json(['otp' => $otp]);
}
