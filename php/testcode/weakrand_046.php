<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_openssl_strong_verified
function weakrand046(BenchmarkRequest $req): BenchmarkResponse {
    $bytes = openssl_random_pseudo_bytes(32, $isStrong);
    if ($isStrong !== true) { // vuln-code-snippet safe-line php_weakrand_openssl_strong_verified
        return BenchmarkResponse::error('not strong');
    }
    $token = bin2hex($bytes);
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_openssl_strong_verified
