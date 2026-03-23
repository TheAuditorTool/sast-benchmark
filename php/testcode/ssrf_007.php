<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_get_headers
function ssrf_get_headers(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $headers = get_headers($url); // vuln-code-snippet vuln-line php_ssrf_get_headers
    return BenchmarkResponse::json($headers);
}
// vuln-code-snippet end php_ssrf_get_headers
