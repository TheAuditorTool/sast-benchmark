<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_curl_open_url
function ssrf017(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url); // vuln-code-snippet vuln-line php_ssrf_curl_open_url
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_curl_open_url
