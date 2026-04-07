<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_variable_function_get
function unsafereflect033(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $action($req->param('data')); // vuln-code-snippet vuln-line php_reflect_variable_function_get
    return BenchmarkResponse::ok('executed');
}
// vuln-code-snippet end php_reflect_variable_function_get
