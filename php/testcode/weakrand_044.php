<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_bytes_otp
function weakrand044(BenchmarkRequest $req): BenchmarkResponse {
    $otp = strtoupper(bin2hex(random_bytes(6))); // vuln-code-snippet safe-line php_weakrand_random_bytes_otp
    return BenchmarkResponse::ok($otp);
}
// vuln-code-snippet end php_weakrand_random_bytes_otp
