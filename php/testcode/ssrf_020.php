<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_get_headers_user
function ssrf020(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $headers = get_headers($url); // vuln-code-snippet vuln-line php_ssrf_get_headers_user
    if ($headers === false) {
        return BenchmarkResponse::badRequest('unreachable');
    }
    return BenchmarkResponse::ok(implode("\n", $headers));
}
// vuln-code-snippet end php_ssrf_get_headers_user
