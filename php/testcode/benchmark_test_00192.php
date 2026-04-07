<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00192(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $img  = imagecreatefromjpeg($file['tmp_name']);
    if ($img === false) {
        return BenchmarkResponse::badRequest('Invalid image');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($img, $dest);
    imagedestroy($img);
    return BenchmarkResponse::ok('Image stored');
}
