<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00506(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('File uploaded: ' . $file['name']);
}
