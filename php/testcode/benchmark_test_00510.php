<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00510(BenchmarkRequest $req): BenchmarkResponse {
    $staticSalt = 'changeme123';
    $key = md5($staticSalt);
    $payload = $req->post('payload');
    $sig = $req->post('sig');
    $expected = hash_hmac('sha256', $payload, $key);
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('invalid signature');
    }
    $obj = unserialize($payload);
    return BenchmarkResponse::ok('processed');
}
