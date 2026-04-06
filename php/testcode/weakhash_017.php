<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_sha512
function weakhash017(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $key = getenv('HMAC_KEY');
    $mac = hash_hmac('sha512', $data, $key); // vuln-code-snippet safe-line php_weakhash_hmac_sha512
    return BenchmarkResponse::json(['mac' => $mac]);
}
// vuln-code-snippet end php_weakhash_hmac_sha512
