<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_relative_only
function redirect012(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    if (str_contains($path, '://') || str_starts_with($path, '//')) {
        return BenchmarkResponse::badRequest('Absolute URLs not allowed');
    }
    header("Location: /" . ltrim($path, '/')); // vuln-code-snippet safe-line php_redirect_relative_only
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_relative_only
