<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_private_range_block
function ssrf014(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $host = parse_url($url, PHP_URL_HOST);
    $ip = gethostbyname($host);
    if (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE) === false) { // vuln-code-snippet safe-line php_ssrf_private_range_block
        return BenchmarkResponse::badRequest('Private/reserved IP not allowed');
    }
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_private_range_block
