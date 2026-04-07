<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00566(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $exists = file_exists("phar://" . $path);
    return BenchmarkResponse::ok($exists ? 'found' : 'not found');
}
