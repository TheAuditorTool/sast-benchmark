<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_dynamic_func
function variablevars009(BenchmarkRequest $req): BenchmarkResponse {
    $funcname = $req->param('action');
    $result = $$funcname(); // vuln-code-snippet vuln-line php_vv_dynamic_func
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_vv_dynamic_func
