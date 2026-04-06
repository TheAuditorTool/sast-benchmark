<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_predictable_token
function csrf008(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->cookie('user_id');
    $token = md5($userId); // vuln-code-snippet vuln-line php_csrf_predictable_token
    if ($req->post('csrf_token') !== $token) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    return BenchmarkResponse::ok('Action performed');
}
// vuln-code-snippet end php_csrf_predictable_token
