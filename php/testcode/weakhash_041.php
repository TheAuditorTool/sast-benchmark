<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_blake2b_keyed
function weakhash041(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $key = random_bytes(SODIUM_CRYPTO_GENERICHASH_KEYBYTES);
    $hash = sodium_crypto_generichash($data, $key); // vuln-code-snippet safe-line php_weakhash_blake2b_keyed
    return BenchmarkResponse::ok(bin2hex($hash));
}
// vuln-code-snippet end php_weakhash_blake2b_keyed
