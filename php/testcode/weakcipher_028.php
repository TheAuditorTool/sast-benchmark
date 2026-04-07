<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_base64_not_encrypt
function weakcipher028(BenchmarkRequest $req): BenchmarkResponse {
    $sensitive = $req->param('data');
    $encoded = base64_encode($sensitive); // vuln-code-snippet vuln-line php_weakcipher_base64_not_encrypt
    return BenchmarkResponse::ok($encoded);
}
// vuln-code-snippet end php_weakcipher_base64_not_encrypt
