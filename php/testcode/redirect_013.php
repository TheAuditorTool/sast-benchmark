<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_startswith_slash
function redirect013(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $clean = '/' . ltrim($path, '/');
    if (str_starts_with($clean, '//')) { // vuln-code-snippet safe-line php_redirect_startswith_slash
        return BenchmarkResponse::badRequest('Invalid path');
    }
    header("Location: " . $clean);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_startswith_slash
