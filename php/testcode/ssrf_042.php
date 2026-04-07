<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_hardcoded_base_only
function ssrf042(BenchmarkRequest $req): BenchmarkResponse {
    $qs = http_build_query(['q' => $req->param('q')]);
    $url = 'https://api.example.com/search?' . $qs;
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_ssrf_hardcoded_base_only
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_hardcoded_base_only
