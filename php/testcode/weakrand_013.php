<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_int_otp
function weakrand013(BenchmarkRequest $req): BenchmarkResponse {
    $otp = random_int(100000, 999999); // vuln-code-snippet safe-line php_weakrand_random_int_otp
    return BenchmarkResponse::json(['otp' => (string)$otp]);
}
// vuln-code-snippet end php_weakrand_random_int_otp
