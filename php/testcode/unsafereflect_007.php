<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_call_user_func_array
function unsafereflect007(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('callback');
    $args = explode(',', $req->param('args'));
    $result = call_user_func_array($func, $args); // vuln-code-snippet vuln-line php_reflect_call_user_func_array
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_reflect_call_user_func_array
