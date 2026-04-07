<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00254(BenchmarkRequest $req): BenchmarkResponse {
    $mime = $req->param('type');
    header(sprintf('Content-Type: %s', $mime));
    return BenchmarkResponse::ok('hello');
}
