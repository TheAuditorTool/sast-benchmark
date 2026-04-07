<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_blowfish_static_iv
function weakcipher032(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(16);
    $staticIv = 'staticiv12345678';
    $enc = openssl_encrypt($req->param('data'), 'BF-CBC', $key, 0, $staticIv); // vuln-code-snippet vuln-line php_weakcipher_blowfish_static_iv
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_blowfish_static_iv
