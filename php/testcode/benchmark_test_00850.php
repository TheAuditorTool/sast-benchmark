<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00850(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Frame-Options: DENY');
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) {
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
