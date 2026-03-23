<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha256
function weakhash006(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $hashed = hash('sha256', $data); // vuln-code-snippet safe-line php_weakhash_sha256
    return BenchmarkResponse::json(['hash' => $hashed]);
}
// vuln-code-snippet end php_weakhash_sha256
