<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00409(BenchmarkRequest $req): BenchmarkResponse {
    $cookieToken = $req->cookie('csrf_token');
    $formToken = $req->post('csrf_token');
    if (empty($cookieToken) || !hash_equals($cookieToken, $formToken)) {
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    return BenchmarkResponse::ok('Action performed');
}
