<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00986(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $name = $file['name'];
    $ext = pathinfo($name, PATHINFO_EXTENSION);
    if ($ext !== 'jpg' && $ext !== 'png') {
        return BenchmarkResponse::badRequest('Only images allowed');
    }
    $dest = '/var/www/uploads/' . $name;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
