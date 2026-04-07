<?php
require_once __DIR__ . '/shared.php';

define('APP_SERVICE_CLASS', 'UserService');

function benchmarkTest00877(BenchmarkRequest $req): BenchmarkResponse {
    $r = new ReflectionClass(APP_SERVICE_CLASS);
    $methods = $r->getMethods(ReflectionMethod::IS_PUBLIC);
    return BenchmarkResponse::ok(count($methods) . ' public methods');
}
