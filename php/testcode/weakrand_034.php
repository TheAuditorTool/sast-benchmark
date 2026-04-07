<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_openssl_strong_check
function weakrand034(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $strong); // vuln-code-snippet safe-line php_weakrand_openssl_strong_check
    if (!$strong) {
        throw new RuntimeException('not strong');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_openssl_strong_check
