<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_md5_derived_key
function weakcipher029(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->param('pass');
    $key = md5($password); // vuln-code-snippet vuln-line php_weakcipher_md5_derived_key
    $enc = openssl_encrypt($req->param('data'), 'AES-256-CBC', $key, 0, random_bytes(16));
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_md5_derived_key
