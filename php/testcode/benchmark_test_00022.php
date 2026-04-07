<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00022(BenchmarkRequest $req): BenchmarkResponse {
    $handler = $req->post('handler');
    call_user_func_array($handler, [$req->param('arg')]);
    return BenchmarkResponse::ok('handled');
}
