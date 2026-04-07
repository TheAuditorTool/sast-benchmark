<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00568(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
