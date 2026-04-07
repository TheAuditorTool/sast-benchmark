<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00710(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $method = $req->param('method');
    $args = $req->param('args');
    $result = call_user_func([$className, $method], $args);
    return BenchmarkResponse::ok((string) $result);
}
