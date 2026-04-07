<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_subdomain_wildcard
function redirect027(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    if (str_ends_with((string) $host, '.example.com')) {
        header('Location: ' . $url); // vuln-code-snippet vuln-line php_redirect_subdomain_wildcard
    }
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_subdomain_wildcard
