<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00812(BenchmarkRequest $req): BenchmarkResponse {
    $handlerClass = $req->post('handler');
    $handler = new $handlerClass();
    return BenchmarkResponse::ok($handler->run());
}
