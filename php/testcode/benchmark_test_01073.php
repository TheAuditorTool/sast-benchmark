<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01073(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('class');
    $ref = new ReflectionClass($input);
    $obj = $ref->newInstance();
    return BenchmarkResponse::ok(get_class($obj));
}
