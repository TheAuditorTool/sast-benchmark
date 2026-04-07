<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_gopher_protocol
function ssrf024(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, 'gopher://' . $host . ':6379/_INFO'); // vuln-code-snippet vuln-line php_ssrf_gopher_protocol
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_gopher_protocol
