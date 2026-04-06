<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_str_replace_crlf
function headerinj014(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $clean = str_replace(["\r", "\n", "\0"], '', $input);
    header("X-Data: " . $clean); // vuln-code-snippet safe-line php_headerinj_str_replace_crlf
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_headerinj_str_replace_crlf
