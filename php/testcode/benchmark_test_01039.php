<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01039(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $secret = getenv('CSRF_SECRET');
    $action = 'transfer';
    $expected = hash_hmac('sha256', session_id() . $action, $secret);
    $submitted = $req->post('csrf_token');
    if (!hash_equals($expected, (string) $submitted)) {
        return BenchmarkResponse::badRequest('CSRF token invalid');
    }
    performTransfer($req->post('amount'), $req->post('to'));
    return BenchmarkResponse::ok('transfer complete');
}
