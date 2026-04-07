<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_rc2_cbc
function weakcipher023(BenchmarkRequest $req): BenchmarkResponse {
    $key = 'weak_key_123456';
    $iv = random_bytes(8);
    $enc = openssl_encrypt($req->param('data'), 'RC2-CBC', $key, 0, $iv); // vuln-code-snippet vuln-line php_weakcipher_rc2_cbc
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_rc2_cbc
