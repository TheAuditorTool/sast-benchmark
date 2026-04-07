<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00977(BenchmarkRequest $req): BenchmarkResponse {
    $file  = $_FILES['f'];
    $finfo = finfo_open(FILEINFO_MIME_TYPE);
    $mime  = finfo_file($finfo, $file['tmp_name']);
    finfo_close($finfo);
    if (!in_array($mime, ['image/jpeg', 'image/png', 'image/gif'], true)) {
        return BenchmarkResponse::badRequest('Invalid file type');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.' . pathinfo($file['name'], PATHINFO_EXTENSION);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
