<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_per_request
function csrf013(BenchmarkRequest $req): BenchmarkResponse {
    $submitted = $req->post('csrf_token');
    $stored = $_SESSION['csrf_token'] ?? '';
    if (empty($submitted) || !hash_equals($stored, $submitted)) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32)); // vuln-code-snippet safe-line php_csrf_per_request
    return BenchmarkResponse::ok('Action performed, token rotated');
}
// vuln-code-snippet end php_csrf_per_request
