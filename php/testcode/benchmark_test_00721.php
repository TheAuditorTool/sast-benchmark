<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00721(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    if ($file['type'] !== 'image/svg+xml') {
        return BenchmarkResponse::badRequest('Not SVG');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
