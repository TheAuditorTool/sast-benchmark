<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01105(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('payload');
    $sig = $req->post('signature');
    $expected = hash_hmac('sha256', $payload, getenv('HMAC_SECRET'));
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('Invalid signature');
    }
    $data = unserialize($payload, ['allowed_classes' => false]);
    return BenchmarkResponse::json($data);
}
