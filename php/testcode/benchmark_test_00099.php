<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00099(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = '/var/app/uploads/' . basename($req->param('zip'));
    $tmpDir = sys_get_temp_dir() . '/' . bin2hex(random_bytes(8));
    mkdir($tmpDir, 0700);
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($tmpDir);
    $zip->close();
    return BenchmarkResponse::ok('processed');
}
