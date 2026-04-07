<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00741(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $checksum = md5_file($path);
    return BenchmarkResponse::json(['checksum' => $checksum]);
}
