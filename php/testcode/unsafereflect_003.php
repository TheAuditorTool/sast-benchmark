<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_call_user_func_class
function unsafereflect003(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $method = $req->param('method');
    $args = $req->param('args');
    $result = call_user_func([$className, $method], $args); // vuln-code-snippet vuln-line php_reflect_call_user_func_class
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_reflect_call_user_func_class
