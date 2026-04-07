<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_webhook_secret
function hardcodedcreds022(BenchmarkRequest $req): BenchmarkResponse {
    $secret = 'whsec_hardcoded_literal_1234567890abcdef'; // vuln-code-snippet vuln-line php_hardcoded_webhook_secret
    $payload = $req->bodyStr();
    $signature = $req->header('X-Webhook-Signature');
    $expected = hash_hmac('sha256', $payload, $secret);
    if (!hash_equals($expected, $signature)) {
        return BenchmarkResponse::badRequest('invalid signature');
    }
    return BenchmarkResponse::ok('webhook accepted');
}
// vuln-code-snippet end php_hardcoded_webhook_secret
