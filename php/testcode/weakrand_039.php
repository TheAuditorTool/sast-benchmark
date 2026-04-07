<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_int_full_range
function weakrand039(BenchmarkRequest $req): BenchmarkResponse {
    $n = random_int(PHP_INT_MIN, PHP_INT_MAX); // vuln-code-snippet safe-line php_weakrand_random_int_full_range
    return BenchmarkResponse::ok((string)$n);
}
// vuln-code-snippet end php_weakrand_random_int_full_range
