<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_per_form_random
function csrf042(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $submitted = $req->post('csrf_token');
    $expected = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($expected, (string) $submitted) || empty($expected)) {
        return BenchmarkResponse::badRequest('CSRF check failed');
    }
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line php_csrf_per_form_random
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_csrf_per_form_random
