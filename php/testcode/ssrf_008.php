<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_scheme_check
function ssrf_scheme_check(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (!isset($parsed['scheme']) || $parsed['scheme'] !== 'https') {
        return BenchmarkResponse::badRequest("Only HTTPS URLs allowed");
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_scheme_check
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_scheme_check
