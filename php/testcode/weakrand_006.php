<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_openssl
function weakrand006(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(openssl_random_pseudo_bytes(32)); // vuln-code-snippet safe-line php_weakrand_openssl
    return BenchmarkResponse::json(['token' => $token]);
}
// vuln-code-snippet end php_weakrand_openssl
