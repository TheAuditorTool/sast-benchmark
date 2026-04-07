<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_double_encoding
function fileinclusion026(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $decoded = urldecode($file);
    if (strpos($decoded, '../') !== false) {
        return BenchmarkResponse::badRequest('Path traversal detected');
    }
    include __DIR__ . '/views/' . $decoded; // vuln-code-snippet vuln-line php_fi_double_encoding
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_fi_double_encoding
