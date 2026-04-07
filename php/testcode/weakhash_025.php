<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_adler32_auth
function weakhash025(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $hash = hash('adler32', $token); // vuln-code-snippet vuln-line php_weakhash_adler32_auth
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_adler32_auth
