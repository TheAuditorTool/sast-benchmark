<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00698(BenchmarkRequest $req): BenchmarkResponse {
    $uploadPath = '/var/www/uploads/';
    $dest = $uploadPath . 'file.dat';
    extract($_FILES);
    return BenchmarkResponse::ok("dest=$dest");
}
