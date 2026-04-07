<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_argon2id_password
function weakhash034(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = password_hash($pass, PASSWORD_ARGON2ID); // vuln-code-snippet safe-line php_weakhash_argon2id_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_argon2id_password
