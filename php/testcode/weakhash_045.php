<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_sha512_webhook
function weakhash045(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->bodyStr();
    $key = getenv('WEBHOOK_SECRET');
    $sig = hash_hmac('sha512', $payload, $key); // vuln-code-snippet safe-line php_weakhash_hmac_sha512_webhook
    return BenchmarkResponse::ok($sig);
}
// vuln-code-snippet end php_weakhash_hmac_sha512_webhook
