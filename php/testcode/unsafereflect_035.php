<?php
require_once __DIR__ . '/shared.php';

interface WorkerInterface {
    public function run(): void;
}

class WorkerService implements WorkerInterface {
    public function run(): void {}
}

// vuln-code-snippet start php_reflect_instanceof_worker
function unsafereflect035(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new WorkerService();
    if (!($obj instanceof WorkerInterface)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_reflect_instanceof_worker
    $obj->run();
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_reflect_instanceof_worker
