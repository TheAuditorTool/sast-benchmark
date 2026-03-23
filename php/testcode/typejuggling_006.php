<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_in_array_strict
function typejuggling006(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('format');
    $allowed = ['json', 'xml', 'csv'];
    if (in_array($input, $allowed, true)) { // vuln-code-snippet safe-line php_tj_in_array_strict
        return BenchmarkResponse::ok("format: $input");
    }
    return BenchmarkResponse::error('invalid format', 400);
}
// vuln-code-snippet end php_tj_in_array_strict
