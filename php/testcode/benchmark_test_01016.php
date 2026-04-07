<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01016(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = $req->param('archive');
    $destDir = $req->param('dest');
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($destDir);
    $zip->close();
    return BenchmarkResponse::ok("Archive extracted");
}
