<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_hardcoded_socket
function ssrf016(BenchmarkRequest $req): BenchmarkResponse {
    $fp = fsockopen('127.0.0.1', 6379, $errno, $errstr, 2); // vuln-code-snippet safe-line php_ssrf_hardcoded_socket
    if ($fp) {
        $key = 'user:' . (int)$req->param('id');
        fwrite($fp, "GET $key\r\n");
        $response = fgets($fp);
        fclose($fp);
        return BenchmarkResponse::ok($response);
    }
    return BenchmarkResponse::error('Redis unavailable');
}
// vuln-code-snippet end php_ssrf_hardcoded_socket
