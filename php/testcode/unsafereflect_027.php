<?php
require_once __DIR__ . '/shared.php';

class MyService {
    private function secret(): string { return 'internal'; }
}

// vuln-code-snippet start php_reflect_reflection_access_bypass
function unsafereflect027(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('m');
    $obj = new MyService();
    $r = new ReflectionMethod($obj, $method); // vuln-code-snippet vuln-line php_reflect_reflection_access_bypass
    $r->setAccessible(true);
    $r->invoke($obj);
    return BenchmarkResponse::ok('invoked');
}
// vuln-code-snippet end php_reflect_reflection_access_bypass
