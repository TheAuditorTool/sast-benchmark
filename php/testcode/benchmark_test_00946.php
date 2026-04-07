<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00946(BenchmarkRequest $req): BenchmarkResponse {
    $value = str_replace(["\r", "\n"], '', $req->param('val'));
    header('X-Value: ' . $value);
    return BenchmarkResponse::ok('header set');
}
