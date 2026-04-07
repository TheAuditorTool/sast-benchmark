<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_domain_assert_allowlist
function redirect042(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $allowed = ['example.com', 'app.example.com'];
    if (!in_array($host, $allowed, true)) { // vuln-code-snippet safe-line php_redirect_domain_assert_allowlist
        return BenchmarkResponse::badRequest('not allowed');
    }
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_domain_assert_allowlist
