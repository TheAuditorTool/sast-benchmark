<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_referer_only_missing
function csrf019(BenchmarkRequest $req): BenchmarkResponse {
    $referer = $req->header('Referer');
    if (!empty($referer) && !str_starts_with($referer, 'https://app.example.com/')) {
        return BenchmarkResponse::badRequest('bad referer');
    }
    performDelete($req->param('id')); // vuln-code-snippet vuln-line php_csrf_referer_only_missing
    return BenchmarkResponse::ok('deleted');
}
// vuln-code-snippet end php_csrf_referer_only_missing
