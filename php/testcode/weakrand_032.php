<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_bytes_32
function weakrand032(BenchmarkRequest $req): BenchmarkResponse {
    $token = random_bytes(32); // vuln-code-snippet safe-line php_weakrand_random_bytes_32
    return BenchmarkResponse::ok(bin2hex($token));
}
// vuln-code-snippet end php_weakrand_random_bytes_32
