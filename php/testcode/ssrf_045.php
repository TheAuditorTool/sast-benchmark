<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_inet_pton_range_check
function ssrf045(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $bin = inet_pton($host);
    if ($bin === false || (ord($bin[0]) & 0xFE) === 0xFE) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $content = file_get_contents('http://' . $host . '/data'); // vuln-code-snippet safe-line php_ssrf_inet_pton_range_check
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_inet_pton_range_check
