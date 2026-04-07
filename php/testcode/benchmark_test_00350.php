<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00350(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $src  = imagecreatefromjpeg($file['tmp_name']);
    if ($src === false) {
        return BenchmarkResponse::badRequest('Invalid JPEG');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($src, $dest, 85);
    imagedestroy($src);
    return BenchmarkResponse::ok('Re-encoded without EXIF');
}
