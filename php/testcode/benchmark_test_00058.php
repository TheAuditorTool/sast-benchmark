<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00058(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $tmpPath = tempnam('/var/www/uploads', 'up_');
    $safePath = $tmpPath . '.dat';
    rename($tmpPath, $safePath);
    move_uploaded_file($file['tmp_name'], $safePath);
    return BenchmarkResponse::ok('Uploaded to controlled path');
}
