<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_hmac_hash_equals
function weakhash047(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->bodyStr();
    $provided = $req->header('X-Signature');
    $expected = hash_hmac('sha256', $payload, getenv('SECRET'));
    if (!hash_equals($expected, $provided)) { // vuln-code-snippet safe-line php_weakhash_hmac_hash_equals
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('verified');
}
// vuln-code-snippet end php_weakhash_hmac_hash_equals
