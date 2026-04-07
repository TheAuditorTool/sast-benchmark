<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_microtime_modulo_otp
function weakrand028(BenchmarkRequest $req): BenchmarkResponse {
    $otp = intval(microtime(true) * 1000) % 100000; // vuln-code-snippet vuln-line php_weakrand_microtime_modulo_otp
    return BenchmarkResponse::ok((string)$otp);
}
// vuln-code-snippet end php_weakrand_microtime_modulo_otp
