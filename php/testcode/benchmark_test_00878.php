<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00878(BenchmarkRequest $req): BenchmarkResponse {
    $cookieToken = $req->cookie('csrf_ds');
    $headerToken = $req->header('X-CSRF-Token');
    if ($cookieToken === $headerToken && !empty($cookieToken)) {
        performTransfer($req->post('amount'), $req->post('to'));
        return BenchmarkResponse::ok('transfer done');
    }
    return BenchmarkResponse::badRequest('CSRF mismatch');
}
