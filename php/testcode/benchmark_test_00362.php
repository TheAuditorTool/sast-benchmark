<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00362(BenchmarkRequest $req): BenchmarkResponse {
    $size = $req->param('size');
    header('Content-Length: ' . (int)$size);
    return BenchmarkResponse::ok(str_repeat('x', (int)$size));
}
