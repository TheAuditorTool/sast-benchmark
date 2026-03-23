<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_curl_hardcoded
function ssrf_curl_hardcoded(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, "https://api.internal.com/status"); // vuln-code-snippet safe-line php_ssrf_curl_hardcoded
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
// vuln-code-snippet end php_ssrf_curl_hardcoded
