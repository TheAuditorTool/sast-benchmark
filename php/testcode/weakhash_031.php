<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha256_no_password_hash
function weakhash031(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = 'fixed_salt';
    $hash = hash('sha256', $salt . $pass); // vuln-code-snippet vuln-line php_weakhash_sha256_no_password_hash
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha256_no_password_hash
