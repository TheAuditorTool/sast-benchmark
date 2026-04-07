<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00984(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $obj = new $className();
    $result = $obj->handle();
    return BenchmarkResponse::ok($result);
}
