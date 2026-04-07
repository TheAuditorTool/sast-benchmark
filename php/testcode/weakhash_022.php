<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha512_no_salt
function weakhash022(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('sha512', $pass); // vuln-code-snippet vuln-line php_weakhash_sha512_no_salt
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha512_no_salt
