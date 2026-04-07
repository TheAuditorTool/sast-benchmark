<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_integer_suffix_only
function variablevars044(BenchmarkRequest $req): BenchmarkResponse {
    $suffix = intval($req->param('id'));
    $varName = 'field_' . $suffix;
    $$varName = $req->param('val'); // vuln-code-snippet safe-line php_vv_integer_suffix_only
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_integer_suffix_only
