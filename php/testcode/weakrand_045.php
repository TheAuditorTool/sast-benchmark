<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_openssl_hardware_rng
function weakrand045(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $strong); // vuln-code-snippet safe-line php_weakrand_openssl_hardware_rng
    if (!$strong) {
        throw new RuntimeException('hardware RNG unavailable');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_openssl_hardware_rng
