<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00953(BenchmarkRequest $req): BenchmarkResponse {
    $otp = random_int(100000, 999999);
    return BenchmarkResponse::ok((string)$otp);
}
