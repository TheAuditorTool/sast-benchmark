<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00029(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dir  = '/var/www/uploads/';
    move_uploaded_file($file['tmp_name'], $dir . $file['name']);
    return BenchmarkResponse::ok('Uploaded as ' . $file['name']);
}
