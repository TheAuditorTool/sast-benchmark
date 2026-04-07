<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_realpath_prefix_check
function fileinclusion035(BenchmarkRequest $req): BenchmarkResponse {
    $f       = $req->param('f');
    $viewsDir = realpath(__DIR__ . '/views') . DIRECTORY_SEPARATOR;
    $real    = realpath($viewsDir . $f);
    if ($real === false || strpos($real, $viewsDir) !== 0) { // vuln-code-snippet safe-line php_fi_realpath_prefix_check
        return BenchmarkResponse::badRequest('Forbidden path');
    }
    include $real;
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_realpath_prefix_check
