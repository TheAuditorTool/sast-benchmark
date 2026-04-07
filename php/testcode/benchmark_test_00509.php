<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00509(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    $class::$method();
    return BenchmarkResponse::ok('dispatched');
}
