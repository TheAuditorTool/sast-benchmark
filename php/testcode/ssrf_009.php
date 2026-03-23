<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_redirect_follow
function ssrf_redirect_follow(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url); // vuln-code-snippet vuln-line php_ssrf_redirect_follow
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, true);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
// vuln-code-snippet end php_ssrf_redirect_follow
