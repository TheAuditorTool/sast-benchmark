<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_in_array_loose
function typejuggling005(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('format');
    $allowed = ['json', 'xml', 'csv'];
    if (in_array($input, $allowed)) { // vuln-code-snippet vuln-line php_tj_in_array_loose
        return BenchmarkResponse::ok("format: $input");
    }
    return BenchmarkResponse::error('invalid format', 400);
}
// vuln-code-snippet end php_tj_in_array_loose
