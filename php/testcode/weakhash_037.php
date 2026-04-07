<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_pbkdf2_100k
function weakhash037(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $salt = random_bytes(32);
    $hash = hash_pbkdf2('sha256', $pass, $salt, 100000); // vuln-code-snippet safe-line php_weakhash_pbkdf2_100k
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_pbkdf2_100k
