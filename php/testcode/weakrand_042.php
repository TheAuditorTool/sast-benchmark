<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_sodium_keygen
function weakrand042(BenchmarkRequest $req): BenchmarkResponse {
    $key = sodium_crypto_generichash_keygen(); // vuln-code-snippet safe-line php_weakrand_sodium_keygen
    return BenchmarkResponse::ok(bin2hex($key));
}
// vuln-code-snippet end php_weakrand_sodium_keygen
