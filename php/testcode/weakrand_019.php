<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_time_entropy
function weakrand019(BenchmarkRequest $req): BenchmarkResponse {
    $token = base64_encode((string)time()); // vuln-code-snippet vuln-line php_weakrand_time_entropy
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_time_entropy
