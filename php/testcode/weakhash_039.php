<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha3_256_general
function weakhash039(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $hash = hash('sha3-256', $data); // vuln-code-snippet safe-line php_weakhash_sha3_256_general
    return BenchmarkResponse::ok($hash);
}
// vuln-code-snippet end php_weakhash_sha3_256_general
