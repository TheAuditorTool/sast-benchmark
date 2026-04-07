<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00326(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $allowed = ['App\\Export', 'App\\Import', 'App\\Report'];
    if (!in_array($className, $allowed, true)) {
        return BenchmarkResponse::badRequest('Handler not allowed');
    }
    $handler = new $className();
    return BenchmarkResponse::ok($handler->run());
}
