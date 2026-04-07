<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_redirect_follow_internal
function ssrf025(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $req->param('url'));
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, true); // vuln-code-snippet vuln-line php_ssrf_redirect_follow_internal
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_redirect_follow_internal
