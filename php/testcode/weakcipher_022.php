<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_aes128_ecb
function weakcipher022(BenchmarkRequest $req): BenchmarkResponse {
    $key = substr(hash('sha256', getenv('KEY')), 0, 16);
    $enc = openssl_encrypt($req->param('data'), 'AES-128-ECB', $key); // vuln-code-snippet vuln-line php_weakcipher_aes128_ecb
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_aes128_ecb
