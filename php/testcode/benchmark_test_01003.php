<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01003(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $val = $req->param('val');
    ${"GLOBALS"}[$key] = $val;
    return BenchmarkResponse::ok('written');
}
