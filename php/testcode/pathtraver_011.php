<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_copy
function pathtraver_copy(BenchmarkRequest $req): BenchmarkResponse {
    $source = $req->param('source');
    $dest = $req->param('dest');
    copy($source, $dest); // vuln-code-snippet vuln-line php_pt_copy
    return BenchmarkResponse::ok("File copied");
}
// vuln-code-snippet end php_pt_copy
