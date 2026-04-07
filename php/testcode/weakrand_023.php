<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_srand_time
function weakrand023(BenchmarkRequest $req): BenchmarkResponse {
    mt_srand(time()); // vuln-code-snippet vuln-line php_weakrand_srand_time
    $token = mt_rand();
    return BenchmarkResponse::ok((string)$token);
}
// vuln-code-snippet end php_weakrand_srand_time
