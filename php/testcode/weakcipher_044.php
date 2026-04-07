<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes256_ctr_hmac
function weakcipher044(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CTR', $key, 0, $iv);
    $mac = hash_hmac('sha256', $iv . $enc, $key); // vuln-code-snippet safe-line php_weakcipher_aes256_ctr_hmac
    return BenchmarkResponse::json(['ct' => $enc, 'mac' => $mac]);
}
// vuln-code-snippet end php_weakcipher_aes256_ctr_hmac
