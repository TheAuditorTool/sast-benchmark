<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00333(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $tmp  = $file['tmp_name'];
    $magic = file_get_contents($tmp, false, null, 0, 3);
    if ($magic !== "\xFF\xD8\xFF") {
        return BenchmarkResponse::badRequest('Not a JPEG');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($tmp, $dest);
    return BenchmarkResponse::ok('Image uploaded');
}
