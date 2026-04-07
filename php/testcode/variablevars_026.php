<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_logical_not_trick
function variablevars026(BenchmarkRequest $req): BenchmarkResponse {
    $admin = $req->param('admin');
    ${!$admin} = true; // vuln-code-snippet vuln-line php_vv_logical_not_trick
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_vv_logical_not_trick
