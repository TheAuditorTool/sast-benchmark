<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_accept_json_bypass
function csrf018(BenchmarkRequest $req): BenchmarkResponse {
    $accept = $req->header('Accept');
    if (str_contains($accept, 'application/json')) { // vuln-code-snippet vuln-line php_csrf_accept_json_bypass
        performStateChange($req->bodyStr());
        return BenchmarkResponse::json(['ok' => true]);
    }
    $token = $req->post('csrf_token');
    if (!verifyCsrfToken($token)) {
        return BenchmarkResponse::badRequest('CSRF check failed');
    }
    performStateChange($req->bodyStr());
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_csrf_accept_json_bypass
