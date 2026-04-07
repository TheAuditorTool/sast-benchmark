<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_class_constant_access
function unsafereflect022(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $val = constant($class . '::SECRET'); // vuln-code-snippet vuln-line php_reflect_class_constant_access
    return BenchmarkResponse::ok((string)$val);
}
// vuln-code-snippet end php_reflect_class_constant_access
