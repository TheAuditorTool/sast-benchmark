<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_create_func_input
function codeinj022(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    // Legacy PHP 7.x pattern
    $fn = create_function('', $code); // vuln-code-snippet vuln-line php_codeinj_create_func_input
    $result = $fn();
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_create_func_input
