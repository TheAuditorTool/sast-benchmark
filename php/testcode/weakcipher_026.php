<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes_null_iv
function weakcipher026(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = str_repeat("\0", 16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_aes_null_iv
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_aes_null_iv
