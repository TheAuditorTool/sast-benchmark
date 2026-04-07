<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_bin2hex_random
function weakrand036(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(16)); // vuln-code-snippet safe-line php_weakrand_bin2hex_random
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_bin2hex_random
