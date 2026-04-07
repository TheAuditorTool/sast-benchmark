<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01033(BenchmarkRequest $req): BenchmarkResponse {
    $secret = 'whsec_hardcoded_literal_1234567890abcdef';
    $payload = $req->bodyStr();
    $signature = $req->header('X-Webhook-Signature');
    $expected = hash_hmac('sha256', $payload, $secret);
    if (!hash_equals($expected, $signature)) {
        return BenchmarkResponse::badRequest('invalid signature');
    }
    return BenchmarkResponse::ok('webhook accepted');
}
