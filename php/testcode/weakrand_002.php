<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_bytes
function weakrand002(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line php_weakrand_random_bytes
    return BenchmarkResponse::json(['session_token' => $token]);
}
// vuln-code-snippet end php_weakrand_random_bytes
