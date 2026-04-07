<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01083(BenchmarkRequest $req): BenchmarkResponse {
    $fn = $req->param('fn');
    $arg = $req->param('arg');
    $result = $fn($arg);
    return BenchmarkResponse::ok((string) $result);
}
