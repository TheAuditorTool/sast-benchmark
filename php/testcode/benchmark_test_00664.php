<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00664(BenchmarkRequest $req): BenchmarkResponse {
    $otp = (int)(lcg_value() * 1000000);
    return BenchmarkResponse::ok((string)$otp);
}
