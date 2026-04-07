<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00076(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $filename = $_POST['filename'];
    $dir      = '/var/www/uploads/';
    move_uploaded_file($file['tmp_name'], $dir . $filename);
    return BenchmarkResponse::ok('Uploaded');
}
