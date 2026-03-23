<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_password_hash
function weakhash004(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = password_hash($password, PASSWORD_DEFAULT); // vuln-code-snippet safe-line php_weakhash_password_hash
    return BenchmarkResponse::json(['hash' => $hashed]);
}
// vuln-code-snippet end php_weakhash_password_hash
