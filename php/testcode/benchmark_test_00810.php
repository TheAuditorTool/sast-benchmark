<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00810(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $integrity = crc32(file_get_contents($filename));
    if ($integrity === (int)$req->param('expected')) {
        include $filename;
    }
    return BenchmarkResponse::ok('done');
}
