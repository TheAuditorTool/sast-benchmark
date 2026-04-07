<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00652(BenchmarkRequest $req): BenchmarkResponse {
    $format = $req->param('format');
    header("Content-Type: " . $format);
    return BenchmarkResponse::ok('{"status":"ok"}');
}
