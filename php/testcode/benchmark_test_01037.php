<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01037(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    forward_static_call([$class, $method]);
    return BenchmarkResponse::ok('dispatched');
}
