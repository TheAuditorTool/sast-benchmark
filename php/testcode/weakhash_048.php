<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_blake2b_hex_output
function weakhash048(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = sodium_bin2hex(sodium_crypto_generichash($data)); // vuln-code-snippet safe-line php_weakhash_blake2b_hex_output
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_blake2b_hex_output
