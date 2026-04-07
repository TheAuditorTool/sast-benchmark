<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha1_static_salt
function weakhash027(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = sha1($pass . 'static_salt_123'); // vuln-code-snippet vuln-line php_weakhash_sha1_static_salt
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha1_static_salt
