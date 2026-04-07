<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00139(BenchmarkRequest $req): BenchmarkResponse {
    $uploadDir = '/var/www/uploads/';
    $htaccess  = $uploadDir . '.htaccess';
    if (!file_exists($htaccess)) {
        file_put_contents($htaccess, "deny from all\n");
    }
    $file = $_FILES['f'];
    $dest = $uploadDir . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
