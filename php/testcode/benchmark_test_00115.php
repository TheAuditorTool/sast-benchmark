<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00115(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $reader = new XMLReader();
    $reader->open($path);
    return BenchmarkResponse::ok('opened');
}
