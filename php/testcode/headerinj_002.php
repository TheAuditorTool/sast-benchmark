<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_sanitized
function headerinj002(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');
    $sanitized = str_replace(["\r", "\n"], '', $input); // vuln-code-snippet safe-line php_headerinj_sanitized
    header("Location: " . $sanitized);
    return BenchmarkResponse::ok('Redirecting...');
}
// vuln-code-snippet end php_headerinj_sanitized
