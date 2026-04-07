<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_ripemd160_password
function weakhash033(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('ripemd160', $pass); // vuln-code-snippet vuln-line php_weakhash_ripemd160_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_ripemd160_password
