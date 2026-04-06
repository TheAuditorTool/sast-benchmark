<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_global_overwrite
function variablevars010(BenchmarkRequest $req): BenchmarkResponse {
    $auth_level = 'none';
    $secret_key = getenv('APP_SECRET');
    $key = $req->param('key');
    $val = $req->param('val');
    $$key = $val; // vuln-code-snippet vuln-line php_vv_global_overwrite
    return BenchmarkResponse::json(['auth' => $auth_level]);
}
// vuln-code-snippet end php_vv_global_overwrite
