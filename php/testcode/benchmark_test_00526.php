<?php
require_once __DIR__ . '/shared.php';

class Processor008 {
    public function export(): string { return 'exported'; }
    public function delete(): string { return 'deleted'; }
}

function benchmarkTest00526(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('action');
    $obj = new Processor008();
    $result = $obj->$method();
    return BenchmarkResponse::ok($result);
}
