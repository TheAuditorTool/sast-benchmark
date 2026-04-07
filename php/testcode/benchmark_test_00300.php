<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00300(BenchmarkRequest $req): BenchmarkResponse {
    $accept = $req->header('Accept');
    if (str_contains($accept, 'application/json')) {
        performStateChange($req->bodyStr());
        return BenchmarkResponse::json(['ok' => true]);
    }
    $token = $req->post('csrf_token');
    if (!verifyCsrfToken($token)) {
        return BenchmarkResponse::badRequest('CSRF check failed');
    }
    performStateChange($req->bodyStr());
    return BenchmarkResponse::ok('done');
}
