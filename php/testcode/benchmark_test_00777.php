<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00777(BenchmarkRequest $req): BenchmarkResponse {
    $otp = strtoupper(bin2hex(random_bytes(6)));
    return BenchmarkResponse::ok($otp);
}
