<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00604(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $zip  = new ZipArchive();
    if ($zip->open($file['tmp_name']) !== true) {
        return BenchmarkResponse::error('Bad zip');
    }
    $zip->extractTo('/var/www/html/');
    $zip->close();
    return BenchmarkResponse::ok('Extracted');
}
