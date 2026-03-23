<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_unlink_basename
function pathtraver_unlink_basename(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $safe = basename($filename);
    unlink("/tmp/" . $safe); // vuln-code-snippet safe-line php_pt_unlink_basename
    return BenchmarkResponse::ok("File deleted");
}
// vuln-code-snippet end php_pt_unlink_basename
