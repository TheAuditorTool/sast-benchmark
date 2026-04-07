<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_rc4_stream
function weakcipher033(BenchmarkRequest $req): BenchmarkResponse {
    $key = random_bytes(16);
    $enc = openssl_encrypt($req->param('data'), 'RC4', $key); // vuln-code-snippet vuln-line php_weakcipher_rc4_stream
    return BenchmarkResponse::ok($enc);
}
// vuln-code-snippet end php_weakcipher_rc4_stream
