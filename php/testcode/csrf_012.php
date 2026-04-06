<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_custom_header
function csrf012(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->header('X-CSRF-Token');
    $sessionToken = $_SESSION['csrf_token'] ?? '';
    if (empty($token) || !hash_equals($sessionToken, $token)) { // vuln-code-snippet safe-line php_csrf_custom_header
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    return BenchmarkResponse::ok('Action performed');
}
// vuln-code-snippet end php_csrf_custom_header
