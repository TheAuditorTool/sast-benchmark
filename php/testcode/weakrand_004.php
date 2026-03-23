<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_int
function weakrand004(BenchmarkRequest $req): BenchmarkResponse {
    $otp = random_int(100000, 999999); // vuln-code-snippet safe-line php_weakrand_random_int
    return BenchmarkResponse::json(['otp' => $otp]);
}
// vuln-code-snippet end php_weakrand_random_int
