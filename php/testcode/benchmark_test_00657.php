<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00657(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Stored outside webroot');
}
