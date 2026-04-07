<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_password_default
function weakhash042(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_DEFAULT); // vuln-code-snippet safe-line php_weakhash_password_default
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_password_default
