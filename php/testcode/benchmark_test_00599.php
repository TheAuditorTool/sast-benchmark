<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00599(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) {
        return BenchmarkResponse::badRequest('CSRF check failed');
    }
    session_regenerate_id(true);
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
