<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_zip_extract_tempdir
function pathtraver036(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = '/var/app/uploads/' . basename($req->param('zip'));
    $tmpDir = sys_get_temp_dir() . '/' . bin2hex(random_bytes(8));
    mkdir($tmpDir, 0700);
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($tmpDir); // vuln-code-snippet safe-line php_pt_zip_extract_tempdir
    $zip->close();
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_pt_zip_extract_tempdir
