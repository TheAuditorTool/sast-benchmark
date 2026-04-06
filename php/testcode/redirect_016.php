<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_domain_allowlist
function redirect016(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $scheme = parse_url($url, PHP_URL_SCHEME);
    $allowedHosts = ['example.com', 'app.example.com', 'cdn.example.com'];
    if (!in_array($scheme, ['http', 'https'], true) || !in_array($host, $allowedHosts, true)) { // vuln-code-snippet safe-line php_redirect_domain_allowlist
        return BenchmarkResponse::badRequest('Redirect not allowed');
    }
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_domain_allowlist
