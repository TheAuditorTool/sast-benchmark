<?php
require_once __DIR__ . '/shared.php';

class KnownService016 {
    public function export(): string { return 'exported'; }
    public function report(): string { return 'reported'; }
}

function benchmarkTest01069(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('method');
    $ref = new ReflectionMethod('KnownService016', $method);
    $obj = new KnownService016();
    $result = $ref->invoke($obj);
    return BenchmarkResponse::ok((string)$result);
}
