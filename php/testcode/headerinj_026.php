<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_content_type_sprintf
function headerinj026(BenchmarkRequest $req): BenchmarkResponse {
    $mime = $req->param('type');
    header(sprintf('Content-Type: %s', $mime)); // vuln-code-snippet vuln-line php_headerinj_content_type_sprintf
    return BenchmarkResponse::ok('hello');
}
// vuln-code-snippet end php_headerinj_content_type_sprintf
