<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01111(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    include 'expect://' . $cmd;
    return BenchmarkResponse::ok('Done');
}
