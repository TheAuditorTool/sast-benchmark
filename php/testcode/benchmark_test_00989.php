<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00989(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('key');
    $val = $req->param('val');
    $GLOBALS[$input] = $val;
    return BenchmarkResponse::ok('written');
}
