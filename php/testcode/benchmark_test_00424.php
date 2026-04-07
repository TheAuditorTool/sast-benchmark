<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00424(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    passthru("convert " . $filename . " output.png");
    return BenchmarkResponse::ok("conversion done");
}
