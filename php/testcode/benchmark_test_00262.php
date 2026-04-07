<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00262(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if ($ext === 'php') {
        return BenchmarkResponse::badRequest('PHP not allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
