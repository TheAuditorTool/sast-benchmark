<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00009(BenchmarkRequest $req): BenchmarkResponse {
    $file    = $_FILES['f'];
    $ext     = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext;
    $dest = '/var/www/uploads/' . $safeName;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
