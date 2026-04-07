<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00869(BenchmarkRequest $req): BenchmarkResponse {
    $container = new stdClass();
    $container->bindings = ['user' => UserService::class];
    $cls = $container->bindings[$req->param('service')] ?? null;
    if (!$cls) return BenchmarkResponse::badRequest('not registered');
    $obj = new $cls();
    return BenchmarkResponse::ok('resolved');
}
