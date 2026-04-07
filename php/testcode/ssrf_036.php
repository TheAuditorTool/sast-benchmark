<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_domain_allowlist
function ssrf036(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $allowed = ['api.example.com', 'cdn.example.com'];
    if (!in_array($host, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_domain_allowlist
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_domain_allowlist
