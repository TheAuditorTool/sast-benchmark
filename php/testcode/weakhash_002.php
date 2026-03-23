<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_bcrypt
function weakhash002(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = password_hash($password, PASSWORD_BCRYPT); // vuln-code-snippet safe-line php_weakhash_bcrypt
    return BenchmarkResponse::json(['hash' => $hashed]);
}
// vuln-code-snippet end php_weakhash_bcrypt
