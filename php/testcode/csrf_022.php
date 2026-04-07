<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_cors_wildcard
function csrf022(BenchmarkRequest $req): BenchmarkResponse {
    header('Access-Control-Allow-Origin: *'); // vuln-code-snippet vuln-line php_csrf_cors_wildcard
    header('Access-Control-Allow-Headers: X-Custom-Token');
    $token = $req->header('X-Custom-Token');
    if (empty($token)) {
        return BenchmarkResponse::badRequest('missing token');
    }
    performAction($req->bodyStr());
    return BenchmarkResponse::json(['ok' => true]);
}
// vuln-code-snippet end php_csrf_cors_wildcard
