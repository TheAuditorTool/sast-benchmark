<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_ctype_alnum_module
function fileinclusion040(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('mod');
    if (!ctype_alnum($module)) { // vuln-code-snippet safe-line php_fi_ctype_alnum_module
        return BenchmarkResponse::badRequest('Invalid module name');
    }
    include __DIR__ . '/modules/' . $module . '/init.php';
    return BenchmarkResponse::ok('Module loaded');
}
// vuln-code-snippet end php_fi_ctype_alnum_module
