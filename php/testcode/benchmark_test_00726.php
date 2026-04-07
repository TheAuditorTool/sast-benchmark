<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00726(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if ($ext !== 'pdf') {
        return BenchmarkResponse::badRequest('Only PDF allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('PDF uploaded');
}
