<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_zip_extract
function pathtraver_zip_extract(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = $req->param('archive');
    $destDir = $req->param('dest');
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($destDir); // vuln-code-snippet vuln-line php_pt_zip_extract
    $zip->close();
    return BenchmarkResponse::ok("Archive extracted");
}
// vuln-code-snippet end php_pt_zip_extract
