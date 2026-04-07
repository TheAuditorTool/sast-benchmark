<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_cors_strict_allowlist
function csrf032(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['https://app.example.com', 'https://admin.example.com'];
    $origin = $req->header('Origin');
    if (!in_array($origin, $allowed, true)) { // vuln-code-snippet safe-line php_csrf_cors_strict_allowlist
        return BenchmarkResponse::badRequest('origin not allowed');
    }
    header('Access-Control-Allow-Origin: ' . $origin);
    performAction($req->bodyStr());
    return BenchmarkResponse::json(['ok' => true]);
}
// vuln-code-snippet end php_csrf_cors_strict_allowlist
