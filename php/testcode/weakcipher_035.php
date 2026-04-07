<?php
require_once __DIR__ . '/shared.php';

define('DES_KEY', 'weakdes!');

// vuln-code-snippet start php_weakcipher_des_global_key
function weakcipher035(BenchmarkRequest $req): BenchmarkResponse {
    $enc = openssl_encrypt($req->param('data'), 'DES-CBC', DES_KEY, 0, random_bytes(8)); // vuln-code-snippet vuln-line php_weakcipher_des_global_key
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_des_global_key
