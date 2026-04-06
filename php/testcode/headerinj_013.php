<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_header_remove
function headerinj013(BenchmarkRequest $req): BenchmarkResponse {
    $value = $req->param('value');
    $sanitized = str_replace(["\r", "\n"], '', $value);
    header_remove('X-Custom');
    header("X-Custom: " . $sanitized); // vuln-code-snippet safe-line php_headerinj_header_remove
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_headerinj_header_remove
