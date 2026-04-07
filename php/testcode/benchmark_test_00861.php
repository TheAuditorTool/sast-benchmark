<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00861(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->bodyStr();
    $provided = $req->header('X-Signature');
    $expected = hash_hmac('sha256', $payload, getenv('SECRET'));
    if (!hash_equals($expected, $provided)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('verified');
}
