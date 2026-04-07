<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_rfc1918_dns_precheck
function ssrf033(BenchmarkRequest $req): BenchmarkResponse {
    $host = parse_url($req->param('url'), PHP_URL_HOST);
    $ip = gethostbyname($host);
    if (filter_var($ip, FILTER_VALIDATE_IP, FILTER_FLAG_NO_PRIV_RANGE | FILTER_FLAG_NO_RES_RANGE) === false) {
        return BenchmarkResponse::badRequest('internal');
    }
    $content = file_get_contents($req->param('url')); // vuln-code-snippet safe-line php_ssrf_rfc1918_dns_precheck
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_rfc1918_dns_precheck
