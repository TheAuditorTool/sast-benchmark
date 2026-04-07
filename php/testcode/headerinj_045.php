<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_str_replace_multichar
function headerinj045(BenchmarkRequest $req): BenchmarkResponse {
    $value = str_replace(["\r", "\n"], '', $req->param('val')); // vuln-code-snippet safe-line php_headerinj_str_replace_multichar
    header('X-Value: ' . $value);
    return BenchmarkResponse::ok('header set');
}
// vuln-code-snippet end php_headerinj_str_replace_multichar
