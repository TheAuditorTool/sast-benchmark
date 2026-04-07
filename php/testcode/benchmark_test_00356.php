<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00356(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('method');
    $obj = new stdClass();
    $obj->$method();
    return BenchmarkResponse::ok('called');
}
