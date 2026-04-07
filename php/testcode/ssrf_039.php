<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_dns_pinning
function ssrf039(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'https://api.example.com/data');
    curl_setopt($ch, CURLOPT_RESOLVE, ['api.example.com:443:1.2.3.4']); // vuln-code-snippet safe-line php_ssrf_dns_pinning
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_dns_pinning
