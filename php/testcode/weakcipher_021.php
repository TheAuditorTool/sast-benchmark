<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_des_cbc
function weakcipher021(BenchmarkRequest $req): BenchmarkResponse {
    $key = substr(hash('sha256', 'static'), 0, 8);
    $iv = random_bytes(8);
    $enc = openssl_encrypt($req->param('data'), 'DES-CBC', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_des_cbc
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_des_cbc
