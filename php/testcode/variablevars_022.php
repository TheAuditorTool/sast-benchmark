<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_globals_overwrite
function variablevars022(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('key');
    $val = $req->param('val');
    $GLOBALS[$input] = $val; // vuln-code-snippet vuln-line php_vv_globals_overwrite
    return BenchmarkResponse::ok('written');
}
// vuln-code-snippet end php_vv_globals_overwrite
