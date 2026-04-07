<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_openssl_sha1_password
function weakhash032(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = openssl_digest($pass, 'SHA1'); // vuln-code-snippet vuln-line php_weakhash_openssl_sha1_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_openssl_sha1_password
