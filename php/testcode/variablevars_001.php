<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_double_dollar
function variablevars001(BenchmarkRequest $req): BenchmarkResponse {
    $varname = $req->param('key');
    $value = $req->param('value');
    $$varname = $value; // vuln-code-snippet vuln-line php_vv_double_dollar
    return BenchmarkResponse::ok("set $varname");
}
// vuln-code-snippet end php_vv_double_dollar
