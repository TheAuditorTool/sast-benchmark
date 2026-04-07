<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01245(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $obj = new $className();
    $result = $obj->process($req->bodyStr());
    return BenchmarkResponse::ok($result);
}
