<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_sha256
function weakhash008(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('HMAC_SECRET');
    $mac = hash_hmac('sha256', $data, $key); // vuln-code-snippet safe-line php_weakhash_hmac_sha256
    return BenchmarkResponse::json(['mac' => $mac]);
}
// vuln-code-snippet end php_weakhash_hmac_sha256
