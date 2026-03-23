<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_allowlist
function ssrf_allowlist(BenchmarkRequest $req): BenchmarkResponse {
    $allowedDomains = ['api.example.com', 'cdn.example.com', 'images.example.com'];
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, $allowedDomains, true)) {
        return BenchmarkResponse::badRequest("Domain not allowed");
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_allowlist
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_allowlist
