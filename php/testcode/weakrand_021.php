<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_lcg_otp
function weakrand021(BenchmarkRequest $req): BenchmarkResponse {
    $otp = (int)(lcg_value() * 1000000); // vuln-code-snippet vuln-line php_weakrand_lcg_otp
    return BenchmarkResponse::ok((string)$otp);
}
// vuln-code-snippet end php_weakrand_lcg_otp
