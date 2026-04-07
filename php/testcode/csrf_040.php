<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_x_frame_deny
function csrf040(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Frame-Options: DENY');
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) { // vuln-code-snippet safe-line php_csrf_x_frame_deny
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_csrf_x_frame_deny
