<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01104(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $ext      = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $safeName = hash_file('sha256', $file['tmp_name']) . '.' . $ext;
    $dest     = '/var/www/uploads/' . $safeName;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Stored as ' . $safeName);
}
