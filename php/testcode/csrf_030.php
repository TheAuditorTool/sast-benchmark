<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_samesite_strict_plus_token
function csrf030(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) { // vuln-code-snippet safe-line php_csrf_samesite_strict_plus_token
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    setcookie('session', session_id(), ['samesite' => 'Strict', 'httponly' => true, 'secure' => true]);
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_csrf_samesite_strict_plus_token
