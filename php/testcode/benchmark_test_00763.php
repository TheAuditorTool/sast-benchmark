<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00763(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $img = imagecreatefromstring(file_get_contents($file['tmp_name']));
    if ($img === false) {
        return BenchmarkResponse::badRequest('Not a valid image');
    }
    $safeName = bin2hex(random_bytes(16)) . '.png';
    imagepng($img, '/var/www/uploads/' . $safeName);
    imagedestroy($img);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
