<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_crc32_microtime_token
function weakrand027(BenchmarkRequest $req): BenchmarkResponse {
    $token = sprintf('%08x', crc32(microtime())); // vuln-code-snippet vuln-line php_weakrand_crc32_microtime_token
    return BenchmarkResponse::ok($token);
}
// vuln-code-snippet end php_weakrand_crc32_microtime_token
