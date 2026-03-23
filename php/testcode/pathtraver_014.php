<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_zip_safe
function pathtraver_zip_safe(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = "/var/uploads/" . basename($req->param('archive'));
    $destDir = "/var/extracted";
    $zip = new ZipArchive();
    $zip->open($zipPath);
    for ($i = 0; $i < $zip->numFiles; $i++) {
        $entryName = $zip->getNameIndex($i);
        if (str_contains($entryName, '..') || str_starts_with($entryName, '/')) {
            continue;
        }
        $zip->extractTo($destDir, $entryName); // vuln-code-snippet safe-line php_pt_zip_safe
    }
    $zip->close();
    return BenchmarkResponse::ok("Archive extracted safely");
}
// vuln-code-snippet end php_pt_zip_safe
