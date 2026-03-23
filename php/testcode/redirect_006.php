<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_parse_url
function redirect006(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $allowedHosts = ['example.com', 'www.example.com'];
    $host = parse_url($url, PHP_URL_HOST);
    if (in_array($host, $allowedHosts, true)) { // vuln-code-snippet safe-line php_redirect_parse_url
        header("Location: " . $url);
        return BenchmarkResponse::ok('Redirecting...');
    }
    return BenchmarkResponse::badRequest('Invalid redirect host');
}
// vuln-code-snippet end php_redirect_parse_url
