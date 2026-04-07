<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_sodium_randombytes
function weakrand035(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_randombytes_buf(32); // vuln-code-snippet safe-line php_weakrand_sodium_randombytes
    return BenchmarkResponse::ok(bin2hex($key));
}
// vuln-code-snippet end php_weakrand_sodium_randombytes
