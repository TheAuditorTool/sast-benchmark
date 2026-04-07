<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_sha256_mac
function weakhash038(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->bodyStr();
    $key = getenv('MAC_SECRET');
    $mac = hash_hmac('sha256', $data, $key); // vuln-code-snippet safe-line php_weakhash_hmac_sha256_mac
    return BenchmarkResponse::ok($mac);
}
// vuln-code-snippet end php_weakhash_hmac_sha256_mac
