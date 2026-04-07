<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_get_key_val_global
function variablevars019(BenchmarkRequest $req): BenchmarkResponse {
    $var = $req->param('var');
    $val = $req->param('val');
    $$var = $val; // vuln-code-snippet vuln-line php_vv_get_key_val_global
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_vv_get_key_val_global
