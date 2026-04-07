<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_controlled_proxy_egress
function ssrf046(BenchmarkRequest $req): BenchmarkResponse {
    $ch = curl_init($req->param('url'));
    curl_setopt($ch, CURLOPT_PROXY, '127.0.0.1:3128'); // vuln-code-snippet safe-line php_ssrf_controlled_proxy_egress
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_ssrf_controlled_proxy_egress
