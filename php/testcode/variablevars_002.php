<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_explicit_assign
function variablevars002(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $value = $req->param('value');
    $config = [];
    $config[$key] = $value; // vuln-code-snippet safe-line php_vv_explicit_assign
    return BenchmarkResponse::json($config);
}
// vuln-code-snippet end php_vv_explicit_assign
