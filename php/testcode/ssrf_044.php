<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_config_url_no_user
function ssrf044(BenchmarkRequest $req): BenchmarkResponse {
    $apiUrl = getenv('UPSTREAM_API_URL');
    $content = file_get_contents($apiUrl . '/health'); // vuln-code-snippet safe-line php_ssrf_config_url_no_user
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_config_url_no_user
