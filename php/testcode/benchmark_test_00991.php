<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00991(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $info = getimagesize($file['tmp_name']);
    if ($info === false || $info[0] <= 0 || $info[1] <= 0) {
        return BenchmarkResponse::badRequest('Invalid image dimensions');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.img';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Image uploaded');
}
