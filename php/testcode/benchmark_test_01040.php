<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01040(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['upload'] ?? null;
    if (!$file || $file['error'] !== UPLOAD_ERR_OK) {
        return BenchmarkResponse::badRequest('no file');
    }
    $dest = '/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('uploaded: ' . $dest);
}
