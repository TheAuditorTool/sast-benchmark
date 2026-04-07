<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00200(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    pcntl_exec('/usr/bin/ls', [escapeshellarg($dir)]);
    return BenchmarkResponse::ok('executed');
}
