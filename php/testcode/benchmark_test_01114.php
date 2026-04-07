<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01114(BenchmarkRequest $req): BenchmarkResponse {
    $secret = getenv('DESER_SECRET');
    $payload = $req->post('payload');
    $sig = $req->post('sig');
    $expected = hash_hmac('sha256', $payload, $secret);
    if (!hash_equals($expected, (string) $sig)) {
        return BenchmarkResponse::badRequest('invalid signature');
    }
    $obj = unserialize($payload);
    return BenchmarkResponse::ok('processed');
}
