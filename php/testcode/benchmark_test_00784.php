<?php
require_once __DIR__ . '/shared.php';

define('SAFE_DIR', '/var/app/data');

function benchmarkTest00784(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    chdir(SAFE_DIR);
    $content = file_get_contents($filename);
    return BenchmarkResponse::ok($content);
}
