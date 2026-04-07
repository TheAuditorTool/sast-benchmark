<?php
require_once __DIR__ . '/shared.php';

define('APP_SERVICE_CLASS', 'UserService');

// vuln-code-snippet start php_reflect_hardcoded_constant_class
function unsafereflect047(BenchmarkRequest $req): BenchmarkResponse {
    $r = new ReflectionClass(APP_SERVICE_CLASS); // vuln-code-snippet safe-line php_reflect_hardcoded_constant_class
    $methods = $r->getMethods(ReflectionMethod::IS_PUBLIC);
    return BenchmarkResponse::ok(count($methods) . ' public methods');
}
// vuln-code-snippet end php_reflect_hardcoded_constant_class
