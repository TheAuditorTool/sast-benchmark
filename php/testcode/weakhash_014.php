<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha3
function weakhash014(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $digest = hash('sha3-256', $data); // vuln-code-snippet safe-line php_weakhash_sha3
    return BenchmarkResponse::json(['hash' => $digest]);
}
// vuln-code-snippet end php_weakhash_sha3
