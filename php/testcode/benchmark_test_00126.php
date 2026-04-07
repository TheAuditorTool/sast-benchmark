<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00126(BenchmarkRequest $req): BenchmarkResponse {
    $value = preg_replace('/[\r\n]/', '', $req->param('val'));
    header('X-Custom: ' . $value);
    return BenchmarkResponse::ok('header set');
}
