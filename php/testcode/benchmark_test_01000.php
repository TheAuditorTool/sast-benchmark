<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01000(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.bin';
    move_uploaded_file($file['tmp_name'], $dest);
    header('Content-Disposition: attachment; filename="safe.pdf"');
    header('Content-Type: application/octet-stream');
    readfile($dest);
    return BenchmarkResponse::ok('');
}
