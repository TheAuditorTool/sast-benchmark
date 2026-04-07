<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00793(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $imageExts = ['jpg', 'png', 'gif', 'jpeg'];
    if (!in_array($ext, $imageExts, true)) {
        return BenchmarkResponse::badRequest('images only');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
