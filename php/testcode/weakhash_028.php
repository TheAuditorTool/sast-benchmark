<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_md5_api
function weakhash028(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->bodyStr();
    $key = getenv('API_SECRET');
    $sig = hash_hmac('md5', $data, $key); // vuln-code-snippet vuln-line php_weakhash_hmac_md5_api
    return BenchmarkResponse::ok($sig);
}
// vuln-code-snippet end php_weakhash_hmac_md5_api
