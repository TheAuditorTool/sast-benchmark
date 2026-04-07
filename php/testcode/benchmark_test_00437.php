<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00437(BenchmarkRequest $req): BenchmarkResponse {
    $otp = intval(microtime(true) * 1000) % 100000;
    return BenchmarkResponse::ok((string)$otp);
}
