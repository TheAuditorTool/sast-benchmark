<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_rawurlencode_filename
function headerinj035(BenchmarkRequest $req): BenchmarkResponse {
    $filename = rawurlencode($req->param('file')); // vuln-code-snippet safe-line php_headerinj_rawurlencode_filename
    header('Content-Disposition: attachment; filename="' . $filename . '"');
    return BenchmarkResponse::ok('download ready');
}
// vuln-code-snippet end php_headerinj_rawurlencode_filename
