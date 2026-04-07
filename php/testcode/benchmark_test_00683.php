<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00683(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $info = getimagesize($file['tmp_name']);
    if ($info === false) {
        return BenchmarkResponse::badRequest('Not an image');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Image uploaded');
}
