<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01201(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('avatar');
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('uploaded');
}
