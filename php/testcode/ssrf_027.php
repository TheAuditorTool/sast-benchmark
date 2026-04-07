<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_stream_fopen
function ssrf027(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $fp = fopen($url, 'r'); // vuln-code-snippet vuln-line php_ssrf_stream_fopen
    $content = stream_get_contents($fp);
    fclose($fp);
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_stream_fopen
