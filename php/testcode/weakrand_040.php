<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_base64_random
function weakrand040(BenchmarkRequest $req): BenchmarkResponse {
    $token = base64_encode(random_bytes(32)); // vuln-code-snippet safe-line php_weakrand_base64_random
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_base64_random
