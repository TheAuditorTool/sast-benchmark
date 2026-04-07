<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_mt_rand_reset
function weakrand017(BenchmarkRequest $req): BenchmarkResponse {
    $token = mt_rand(100000, 999999); // vuln-code-snippet vuln-line php_weakrand_mt_rand_reset
    return BenchmarkResponse::ok((string)$token);
}
// vuln-code-snippet end php_weakrand_mt_rand_reset
