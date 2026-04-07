<?php
require_once __DIR__ . '/shared.php';

class MyService046 {
    public function run(): string { return 'ok'; }
}

function benchmarkTest00438(BenchmarkRequest $req): BenchmarkResponse {
    $ref = new ReflectionClass(MyService046::class);
    $methods = array_map(fn($m) => $m->getName(), $ref->getMethods());
    return BenchmarkResponse::ok(implode(',', $methods));
}
