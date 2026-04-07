<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakcipher_rot13_security
function weakcipher031(BenchmarkRequest $req): BenchmarkResponse {
    $sensitive = $req->param('secret');
    $obfuscated = str_rot13($sensitive); // vuln-code-snippet vuln-line php_weakcipher_rot13_security
    return BenchmarkResponse::ok($obfuscated);
}
// vuln-code-snippet end php_weakcipher_rot13_security
