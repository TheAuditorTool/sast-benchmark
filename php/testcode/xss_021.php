<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_error_reflection
function xss_error_reflection(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');

    $html = '<html><body><p>Error: page "' . $page . '" not found.</p></body></html>'; // vuln-code-snippet vuln-line php_xss_error_reflection

    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_error_reflection
