<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01100(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = "/var/uploads/" . basename($req->param('archive'));
    $destDir = "/var/extracted";
    $zip = new ZipArchive();
    $zip->open($zipPath);
    for ($i = 0; $i < $zip->numFiles; $i++) {
        $entryName = $zip->getNameIndex($i);
        if (str_contains($entryName, '..') || str_starts_with($entryName, '/')) {
            continue;
        }
        $zip->extractTo($destDir, $entryName);
    }
    $zip->close();
    return BenchmarkResponse::ok("Archive extracted safely");
}
