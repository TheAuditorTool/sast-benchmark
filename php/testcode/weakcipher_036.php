<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes256_gcm_aead
function weakcipher036(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(12);
    $tag = '';
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, $iv, $tag); // vuln-code-snippet safe-line php_weakcipher_aes256_gcm_aead
    return BenchmarkResponse::ok(base64_encode($iv . $tag . $enc));
}
// vuln-code-snippet end php_weakcipher_aes256_gcm_aead
