<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00492(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'png', 'gif', 'pdf'];
    if (!in_array($ext, $allowed, true)) {
        return BenchmarkResponse::badRequest('File type not allowed');
    }
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], '/var/www/uploads/' . $safeName);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
