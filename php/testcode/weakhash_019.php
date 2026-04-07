<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha256_password
function weakhash019(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = hash('sha256', $pass); // vuln-code-snippet vuln-line php_weakhash_sha256_password
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha256_password
