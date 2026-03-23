<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_allowlist
function redirect002(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $allowed = ['/', '/dashboard', '/profile'];
    if (in_array($url, $allowed, true)) { // vuln-code-snippet safe-line php_redirect_allowlist
        header("Location: " . $url);
        return BenchmarkResponse::ok('Redirecting...');
    }
    return BenchmarkResponse::badRequest('Invalid redirect target');
}
// vuln-code-snippet end php_redirect_allowlist
