<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_double_submit_subdomain
function csrf024(BenchmarkRequest $req): BenchmarkResponse {
    $cookieToken = $req->cookie('csrf_ds');
    $headerToken = $req->header('X-CSRF-Token');
    if ($cookieToken === $headerToken && !empty($cookieToken)) { // vuln-code-snippet vuln-line php_csrf_double_submit_subdomain
        performTransfer($req->post('amount'), $req->post('to'));
        return BenchmarkResponse::ok('transfer done');
    }
    return BenchmarkResponse::badRequest('CSRF mismatch');
}
// vuln-code-snippet end php_csrf_double_submit_subdomain
