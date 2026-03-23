<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_file_get_contents
function ssrf_file_get_contents(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $content = file_get_contents($url); // vuln-code-snippet vuln-line php_ssrf_file_get_contents
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_file_get_contents
