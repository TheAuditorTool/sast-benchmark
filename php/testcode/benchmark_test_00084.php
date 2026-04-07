<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00084(BenchmarkRequest $req): BenchmarkResponse {
    $otp = random_int(100000, 999999);
    return BenchmarkResponse::json(['otp' => (string)$otp]);
}
