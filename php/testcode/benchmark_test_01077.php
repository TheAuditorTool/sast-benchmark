<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01077(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    if ($file['type'] !== 'image/jpeg' && $file['type'] !== 'image/png') {
        return BenchmarkResponse::badRequest('only images allowed');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
