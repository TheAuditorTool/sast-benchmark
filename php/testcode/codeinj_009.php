<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_call_user_func
function codeinj_call_user_func(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('func');
    $args = $req->param('args');
    $result = call_user_func($func, $args); // vuln-code-snippet vuln-line php_codeinj_call_user_func
    return BenchmarkResponse::ok("Result: " . $result);
}
// vuln-code-snippet end php_codeinj_call_user_func
