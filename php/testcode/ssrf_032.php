<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_scheme_https_only
function ssrf032(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    if (parse_url($url, PHP_URL_SCHEME) !== 'https') {
        return BenchmarkResponse::badRequest('only https');
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_scheme_https_only
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_scheme_https_only
