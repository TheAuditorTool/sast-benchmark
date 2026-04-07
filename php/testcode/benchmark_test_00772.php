<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00772(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if (!in_array($ext, ['jpg', 'png', 'gif'], true)) {
        return BenchmarkResponse::badRequest('Extension not allowed');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
