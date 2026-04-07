<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_filter_validate_host
function redirect039(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (!filter_var($url, FILTER_VALIDATE_URL)) {
        return BenchmarkResponse::badRequest('invalid url');
    }
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, ['example.com'], true)) {
        return BenchmarkResponse::badRequest('host not allowed');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_filter_validate_host
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_filter_validate_host
