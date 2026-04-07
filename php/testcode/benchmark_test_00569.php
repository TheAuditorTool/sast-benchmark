<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00569(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $secret = getenv('CSRF_SECRET');
    $submitted = $req->post('csrf_token');
    [$ts, $mac] = array_pad(explode('.', (string) $submitted, 2), 2, '');
    $expected = hash_hmac('sha256', session_id() . $ts, $secret);
    if (!hash_equals($expected, $mac) || (time() - (int) $ts) > 300) {
        return BenchmarkResponse::badRequest('token expired or invalid');
    }
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
