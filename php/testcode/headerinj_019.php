<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_crlf_filename
function headerinj019(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    header('Content-Disposition: attachment; filename="' . $name . '"'); // vuln-code-snippet vuln-line php_headerinj_crlf_filename
    return BenchmarkResponse::ok('download ready');
}
// vuln-code-snippet end php_headerinj_crlf_filename
