<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_constant_path_validated
function fileinclusion042(BenchmarkRequest $req): BenchmarkResponse {
    $module   = $req->param('module');
    $allowed  = ['auth', 'dashboard', 'reports'];
    if (!in_array($module, $allowed, true)) { // vuln-code-snippet safe-line php_fi_constant_path_validated
        return BenchmarkResponse::badRequest('Invalid module');
    }
    include constant('APP_PATH') . '/modules/' . $module . '/init.php';
    return BenchmarkResponse::ok('Module loaded');
}
// vuln-code-snippet end php_fi_constant_path_validated
