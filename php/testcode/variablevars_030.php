<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_globals_curly_inject
function variablevars030(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $val = $req->param('val');
    ${"GLOBALS"}[$key] = $val; // vuln-code-snippet vuln-line php_vv_globals_curly_inject
    return BenchmarkResponse::ok('written');
}
// vuln-code-snippet end php_vv_globals_curly_inject
