<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00111(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $header = file_get_contents($file['tmp_name'], false, null, 0, 4);
    if ($header !== "\xFF\xD8\xFF\xE0" && $header !== "GIF8") {
        return BenchmarkResponse::badRequest('Invalid image header');
    }
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
