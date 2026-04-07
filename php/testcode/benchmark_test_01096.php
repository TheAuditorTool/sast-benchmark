<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01096(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    call_user_func([$class, $method]);
    return BenchmarkResponse::ok('dispatched');
}
