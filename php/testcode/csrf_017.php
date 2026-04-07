<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_session_fixed_token
function csrf017(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    if (!isset($_SESSION['csrf_token'])) {
        $_SESSION['csrf_token'] = bin2hex(random_bytes(16));
    }
    $submitted = $req->post('csrf_token');
    if ($submitted === $_SESSION['csrf_token']) { // vuln-code-snippet vuln-line php_csrf_session_fixed_token
        performSensitiveAction();
        return BenchmarkResponse::ok('action done');
    }
    return BenchmarkResponse::badRequest('invalid token');
}
// vuln-code-snippet end php_csrf_session_fixed_token
