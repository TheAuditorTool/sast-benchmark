<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_dns_rebinding
function ssrf023(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $ip = gethostbyname(parse_url($url, PHP_URL_HOST));
    if (strpos($ip, '192.168.') === 0) {
        return BenchmarkResponse::badRequest('internal');
    }
    $content = file_get_contents($req->param('url')); // vuln-code-snippet vuln-line php_ssrf_dns_rebinding
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_dns_rebinding
