<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01177(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $base = '/var/app/data/';
    $resolved = realpath($base . $filename);
    if ($resolved === false || strpos($resolved, $base) !== 0) {
        return BenchmarkResponse::error('not found');
    }
    return BenchmarkResponse::ok(file_get_contents($resolved));
}
