<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_call_validated
function codeinj_call_validated(BenchmarkRequest $req): BenchmarkResponse {
    $allowedFuncs = ['strlen', 'strtolower', 'trim'];
    $func = $req->param('func');
    $args = $req->param('args');
    if (!in_array($func, $allowedFuncs, true)) {
        return BenchmarkResponse::badRequest("Function not allowed");
    }
    $result = call_user_func($func, $args); // vuln-code-snippet safe-line php_codeinj_call_validated
    return BenchmarkResponse::ok("Result: " . $result);
}
// vuln-code-snippet end php_codeinj_call_validated
