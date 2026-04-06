<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_bin2hex
function weakrand014(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line php_weakrand_bin2hex
    return BenchmarkResponse::json(['api_key' => $token]);
}
// vuln-code-snippet end php_weakrand_bin2hex
