<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_dynamic_method_call
function unsafereflect020(BenchmarkRequest $req): BenchmarkResponse {
    $method = $req->param('method');
    $obj = new stdClass();
    $obj->$method(); // vuln-code-snippet vuln-line php_reflect_dynamic_method_call
    return BenchmarkResponse::ok('called');
}
// vuln-code-snippet end php_reflect_dynamic_method_call
