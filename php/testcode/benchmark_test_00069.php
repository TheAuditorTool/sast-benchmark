<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00069(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $mimeType = $file['type'];
    if (!in_array($mimeType, ['image/jpeg', 'image/png'], true)) {
        return BenchmarkResponse::badRequest('Invalid MIME type');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Image uploaded');
}
