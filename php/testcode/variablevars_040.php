<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_read_only_display_scope
function variablevars040(BenchmarkRequest $req): BenchmarkResponse {
    $displayVars = ['color' => $req->param('color')];
    extract($displayVars); // vuln-code-snippet safe-line php_vv_read_only_display_scope
    echo $color;
    return BenchmarkResponse::ok('rendered');
}
// vuln-code-snippet end php_vv_read_only_display_scope
