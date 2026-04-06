<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_define_hardcoded
function variablevars014(BenchmarkRequest $req): BenchmarkResponse {
    $version = $req->param('version');
    define('APP_VERSION', $version); // vuln-code-snippet safe-line php_vv_define_hardcoded
    return BenchmarkResponse::ok('Version set to ' . APP_VERSION);
}
// vuln-code-snippet end php_vv_define_hardcoded
