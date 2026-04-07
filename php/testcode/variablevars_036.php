<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_allowlist_var_names
function variablevars036(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $allowed = ['color', 'font', 'size'];
    if (!in_array($name, $allowed, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_vv_allowlist_var_names
    $$name = $req->param('value');
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_allowlist_var_names
