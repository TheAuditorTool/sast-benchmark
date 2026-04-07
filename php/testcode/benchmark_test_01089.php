<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01089(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) {
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    setcookie('session', session_id(), ['samesite' => 'Strict', 'httponly' => true, 'secure' => true]);
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
