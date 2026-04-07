<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_readfile_not_include
function fileinclusion041(BenchmarkRequest $req): BenchmarkResponse {
    $file = basename($req->param('file'));
    readfile('/var/app/public/' . $file); // vuln-code-snippet safe-line php_fi_readfile_not_include
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_fi_readfile_not_include
