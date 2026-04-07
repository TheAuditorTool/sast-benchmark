<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_bcrypt_cost12
function weakhash035(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_BCRYPT, ['cost' => 12]); // vuln-code-snippet safe-line php_weakhash_bcrypt_cost12
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_bcrypt_cost12
