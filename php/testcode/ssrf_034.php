<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_no_follow_location
function ssrf034(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init($req->param('url'));
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, false); // vuln-code-snippet safe-line php_ssrf_no_follow_location
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_no_follow_location
