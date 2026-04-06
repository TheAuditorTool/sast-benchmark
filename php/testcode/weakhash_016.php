<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_argon2id
function weakhash016(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hash = password_hash($password, PASSWORD_ARGON2ID); // vuln-code-snippet safe-line php_weakhash_argon2id
    return BenchmarkResponse::json(['hash' => $hash]);
}
// vuln-code-snippet end php_weakhash_argon2id
