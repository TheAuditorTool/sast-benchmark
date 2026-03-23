<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_reflection_api
function unsafereflect005(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('class');
    $ref = new ReflectionClass($input); // vuln-code-snippet vuln-line php_reflect_reflection_api
    $obj = $ref->newInstance();
    return BenchmarkResponse::ok(get_class($obj));
}
// vuln-code-snippet end php_reflect_reflection_api
