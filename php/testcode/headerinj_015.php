<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_rawurlencode
function headerinj015(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    header("X-User-Data: " . rawurlencode($input)); // vuln-code-snippet safe-line php_headerinj_rawurlencode
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_headerinj_rawurlencode
