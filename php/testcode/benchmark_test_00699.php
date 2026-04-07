<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00699(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $src  = imagecreatefromstring(file_get_contents($file['tmp_name']));
    if ($src === false) {
        return BenchmarkResponse::badRequest('Invalid image');
    }
    $thumb = imagescale($src, 200, 200);
    $dest  = '/var/www/uploads/thumb_' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($thumb, $dest);
    imagedestroy($src);
    imagedestroy($thumb);
    return BenchmarkResponse::ok('Thumbnail created');
}
