<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_extract_then_variable_var
function variablevars031(BenchmarkRequest $req): BenchmarkResponse {
    extract($_POST);
    $key = $req->param('key');
    echo $$key; // vuln-code-snippet vuln-line php_vv_extract_then_variable_var
    return BenchmarkResponse::ok('read');
}
// vuln-code-snippet end php_vv_extract_then_variable_var
