<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00485(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    if ($file['type'] !== 'text/plain') {
        return BenchmarkResponse::badRequest('Only text allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
