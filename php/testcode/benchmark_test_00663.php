<?php
require_once __DIR__ . '/shared.php';

interface WorkerInterface {
    public function run(): void;
}

class WorkerService implements WorkerInterface {
    public function run(): void {}
}

function benchmarkTest00663(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new WorkerService();
    if (!($obj instanceof WorkerInterface)) return BenchmarkResponse::badRequest('invalid');
    $obj->run();
    return BenchmarkResponse::ok('done');
}
