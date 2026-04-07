<?php
require_once __DIR__ . '/shared.php';

interface HandlerInterface014 {
    public function handle(): string;
}

function benchmarkTest00988(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    if (!class_exists($className)) {
        return BenchmarkResponse::badRequest('Class not found');
    }
    $obj = new $className();
    if (!($obj instanceof HandlerInterface014)) {
        return BenchmarkResponse::badRequest('Invalid handler type');
    }
    return BenchmarkResponse::ok($obj->handle());
}
