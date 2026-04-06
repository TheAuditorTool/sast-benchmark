<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_origin_referer
function csrf014(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->header('Origin');
    $referer = $req->header('Referer');
    $allowedOrigin = 'https://app.example.com';
    $originOk = ($origin === $allowedOrigin);
    $refererOk = str_starts_with($referer, $allowedOrigin . '/');
    if (!$originOk && !$refererOk) { // vuln-code-snippet safe-line php_csrf_origin_referer
        return BenchmarkResponse::badRequest('Origin validation failed');
    }
    return BenchmarkResponse::ok('Action performed');
}
// vuln-code-snippet end php_csrf_origin_referer
