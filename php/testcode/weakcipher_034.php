<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_padding_oracle_cbc
function weakcipher034(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(32);
    $iv = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, $iv);
    $dec = openssl_decrypt($req->param('ciphertext'), 'AES-256-CBC', $key, 0, $iv);
    if ($dec === false) {
        return BenchmarkResponse::error('padding error'); // vuln-code-snippet vuln-line php_weakcipher_padding_oracle_cbc
    }
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_padding_oracle_cbc
