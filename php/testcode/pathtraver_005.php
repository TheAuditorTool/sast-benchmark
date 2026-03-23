<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_unlink
function pathtraver_unlink(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    unlink("/tmp/" . $filename); // vuln-code-snippet vuln-line php_pt_unlink
    return BenchmarkResponse::ok("File deleted");
}
// vuln-code-snippet end php_pt_unlink
