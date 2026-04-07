<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_mcrypt_3des
function weakcipher024(BenchmarkRequest $req): BenchmarkResponse {
    $key = str_repeat('a', 24);
    // Legacy PHP 5.x pattern
    $enc = mcrypt_encrypt(MCRYPT_3DES, $key, $req->param('data'), MCRYPT_MODE_CBC); // vuln-code-snippet vuln-line php_weakcipher_mcrypt_3des
    return BenchmarkResponse::ok(base64_encode($enc));
}
// vuln-code-snippet end php_weakcipher_mcrypt_3des
