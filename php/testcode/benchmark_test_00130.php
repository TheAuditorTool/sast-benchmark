<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00130(BenchmarkRequest $req): BenchmarkResponse {
    $file   = $_FILES['f'];
    $tmp    = $file['tmp_name'];
    $output = [];
    $code   = 0;
    exec('claamscan --no-summary ' . escapeshellarg($tmp), $output, $code);
    if ($code !== 0) {
        return BenchmarkResponse::badRequest('File rejected by antivirus');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($tmp, $dest);
    return BenchmarkResponse::ok('Clean file uploaded');
}
