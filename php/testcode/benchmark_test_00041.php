<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00041(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = '/var/app/uploads/';
    $filename = $req->post('filename');
    move_uploaded_file($_FILES['file']['tmp_name'], $baseDir . $filename);
    return BenchmarkResponse::ok('uploaded');
}
