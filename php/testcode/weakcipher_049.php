<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_secure_random_key_iv
function weakcipher049(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, OPENSSL_RAW_DATA, substr($iv, 0, 12)); // vuln-code-snippet safe-line php_weakcipher_secure_random_key_iv
    return BenchmarkResponse::ok(base64_encode($enc));
}
// vuln-code-snippet end php_weakcipher_secure_random_key_iv
