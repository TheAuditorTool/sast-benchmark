<?php
require_once __DIR__ . '/shared.php';

class MyService {
    private function secret(): string { return 'internal'; }
}

function benchmarkTest00923(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('m');
    $obj = new MyService();
    $r = new ReflectionMethod($obj, $method);
    $r->setAccessible(true);
    $r->invoke($obj);
    return BenchmarkResponse::ok('invoked');
}
