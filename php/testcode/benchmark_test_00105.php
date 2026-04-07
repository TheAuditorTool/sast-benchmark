<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00105(BenchmarkRequest $req): BenchmarkResponse {
    $zipPath = $req->param('zip');
    $webroot = '/var/www/html/';
    $zip = new ZipArchive();
    $zip->open($zipPath);
    $zip->extractTo($webroot);
    $zip->close();
    return BenchmarkResponse::ok('extracted');
}
