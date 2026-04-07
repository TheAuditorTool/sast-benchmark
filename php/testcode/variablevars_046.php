<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_regex_gated_field_name
function variablevars046(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->param('field');
    if (!preg_match('/^field_[0-9]+$/', $field)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_vv_regex_gated_field_name
    $$field = $req->param('val');
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_regex_gated_field_name
