<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_pbkdf2_sha256_key
function weakcipher039(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(32);
    $key = hash_pbkdf2('sha256', $pass, $salt, 100000, 32, true); // vuln-code-snippet safe-line php_weakcipher_pbkdf2_sha256_key
    $enc = openssl_encrypt($req->param('data'), 'AES-256-GCM', $key, 0, random_bytes(12));
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_pbkdf2_sha256_key
