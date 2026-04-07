<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_overwrite_auth_flag
function variablevars024(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    $input = $req->param('flag');
    $$input = true; // vuln-code-snippet vuln-line php_vv_overwrite_auth_flag
    if ($isAdmin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
// vuln-code-snippet end php_vv_overwrite_auth_flag
