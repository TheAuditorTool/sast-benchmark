<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes256_cbc_hmac
function weakcipher041(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, $iv);
    $mac = hash_hmac('sha256', $enc, $key); // vuln-code-snippet safe-line php_weakcipher_aes256_cbc_hmac
    return BenchmarkResponse::json(['ct' => $enc, 'mac' => $mac]);
}
// vuln-code-snippet end php_weakcipher_aes256_cbc_hmac
