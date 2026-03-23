<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_copy_validated
function pathtraver_copy_validated(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = "/var/www/files";
    $source = realpath($baseDir . "/" . $req->param('source'));
    $dest = realpath(dirname($baseDir . "/" . $req->param('dest'))) . "/" . basename($req->param('dest'));
    if ($source === false || !str_starts_with($source, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid source path");
    }
    if (!str_starts_with($dest, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid dest path");
    }
    copy($source, $dest); // vuln-code-snippet safe-line php_pt_copy_validated
    return BenchmarkResponse::ok("File copied");
}
// vuln-code-snippet end php_pt_copy_validated
