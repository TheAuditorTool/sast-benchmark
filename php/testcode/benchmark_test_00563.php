<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00563(BenchmarkRequest $req): BenchmarkResponse {
    $file    = $_FILES['f'];
    $maxSize = 5 * 1024 * 1024;
    if ($file['size'] > $maxSize) {
        return BenchmarkResponse::badRequest('File too large');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
