<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_fsockopen_user
function ssrf030(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $port = (int)$req->param('port');
    $fp = fsockopen($host, $port); // vuln-code-snippet vuln-line php_ssrf_fsockopen_user
    $response = fread($fp, 4096);
    fclose($fp);
    return BenchmarkResponse::ok($response);
}
// vuln-code-snippet end php_ssrf_fsockopen_user
