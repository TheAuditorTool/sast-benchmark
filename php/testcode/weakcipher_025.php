<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes_gcm_nonce_reuse
function weakcipher025(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $nonce = 'fixed_nonce_1234';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, 0, $nonce); // vuln-code-snippet vuln-line php_weakcipher_aes_gcm_nonce_reuse
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_aes_gcm_nonce_reuse
