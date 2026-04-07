<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01203(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('avatar');
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'jpeg', 'png', 'gif', 'webp'];
    $allowedMime = ['image/jpeg', 'image/png', 'image/gif', 'image/webp'];
    if (!in_array($ext, $allowed, true) || !in_array($file['type'], $allowedMime, true)) {
        return BenchmarkResponse::badRequest('invalid file type');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('uploaded');
}
