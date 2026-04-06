<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_double_submit
function csrf011(BenchmarkRequest $req): BenchmarkResponse {
    $cookieToken = $req->cookie('csrf_token');
    $formToken = $req->post('csrf_token');
    if (empty($cookieToken) || !hash_equals($cookieToken, $formToken)) { // vuln-code-snippet safe-line php_csrf_double_submit
        return BenchmarkResponse::badRequest('CSRF validation failed');
    }
    return BenchmarkResponse::ok('Action performed');
}
// vuln-code-snippet end php_csrf_double_submit
