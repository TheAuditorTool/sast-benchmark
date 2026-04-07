<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00893(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $finfo = finfo_open(FILEINFO_MIME_TYPE);
    $mime = finfo_file($finfo, $file['tmp_name']);
    finfo_close($finfo);
    if (!str_starts_with($mime, 'image/')) {
        return BenchmarkResponse::badRequest('Only images allowed');
    }
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
