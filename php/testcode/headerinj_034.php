<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_crlf_strip
function headerinj034(BenchmarkRequest $req): BenchmarkResponse {
    $value = preg_replace('/[\r\n]/', '', $req->param('val')); // vuln-code-snippet safe-line php_headerinj_crlf_strip
    header('X-Custom: ' . $value);
    return BenchmarkResponse::ok('header set');
}
// vuln-code-snippet end php_headerinj_crlf_strip
