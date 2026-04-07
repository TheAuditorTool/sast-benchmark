<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_remote_http_include
function fileinclusion022(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('module');
    include $module; // vuln-code-snippet vuln-line php_fi_remote_http_include
    return BenchmarkResponse::ok('Module loaded');
}
// vuln-code-snippet end php_fi_remote_http_include
