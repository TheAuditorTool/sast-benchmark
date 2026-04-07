<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00380(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $obj = new $className();
    return BenchmarkResponse::ok('created');
}
