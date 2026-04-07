<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00109(BenchmarkRequest $req): BenchmarkResponse {
    $base = __DIR__ . '/templates';
    $file = $req->param('file');
    require_once $base . '/../' . $file;
    return BenchmarkResponse::ok('Loaded');
}
