<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00494(BenchmarkRequest $req): BenchmarkResponse {
    $suffix = $req->param('type');
    $className = 'Handler' . $suffix;
    $obj = new $className();
    return BenchmarkResponse::ok('dispatched');
}
