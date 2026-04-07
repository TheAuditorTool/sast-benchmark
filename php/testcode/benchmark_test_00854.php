<?php
require_once __DIR__ . '/shared.php';

class MyClass {}

function benchmarkTest00854(BenchmarkRequest $req): BenchmarkResponse {
    $r = new ReflectionClass(MyClass::class);
    $methods = $r->getMethods();
    return BenchmarkResponse::ok(count($methods) . ' methods');
}
