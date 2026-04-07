<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00086(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'png', 'gif'];
    if (!in_array($ext, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid extension');
    }
    $finfo = finfo_open(FILEINFO_MIME_TYPE);
    $mime = finfo_file($finfo, $file['tmp_name']);
    finfo_close($finfo);
    $allowedMimes = ['image/jpeg', 'image/png', 'image/gif'];
    if (!in_array($mime, $allowedMimes, true)) {
        return BenchmarkResponse::badRequest('invalid file type');
    }
    if ($file['size'] > 2 * 1024 * 1024) {
        return BenchmarkResponse::badRequest('file too large');
    }
    $dest = 'uploads/' . bin2hex(random_bytes(16)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
