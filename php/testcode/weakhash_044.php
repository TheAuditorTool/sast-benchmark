<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha256_random_salt_nonauth
function weakhash044(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $salt = random_bytes(32);
    $hash = hash('sha256', $data . $salt); // vuln-code-snippet safe-line php_weakhash_sha256_random_salt_nonauth
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha256_random_salt_nonauth
