<?php
require_once __DIR__ . '/shared.php';

class MyClass {}

// vuln-code-snippet start php_reflect_reflection_literal_class
function unsafereflect038(BenchmarkRequest $req): BenchmarkResponse {
    $r = new ReflectionClass(MyClass::class); // vuln-code-snippet safe-line php_reflect_reflection_literal_class
    $methods = $r->getMethods();
    return BenchmarkResponse::ok(count($methods) . ' methods');
}
// vuln-code-snippet end php_reflect_reflection_literal_class
