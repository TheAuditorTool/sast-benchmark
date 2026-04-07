<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_zip_slip
function pathtraver022(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = $req->param('zip');
    $webroot = '/var/www/html/';
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($webroot); // vuln-code-snippet vuln-line php_pt_zip_slip
    $zip->close();
    return BenchmarkResponse::ok('extracted');
}
// vuln-code-snippet end php_pt_zip_slip
