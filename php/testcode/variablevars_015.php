<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_constant_lookup
function variablevars015(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('const');
    $allowed = ['PHP_INT_MAX', 'PHP_INT_SIZE', 'PHP_VERSION'];
    if (!in_array($name, $allowed, true)) {
        return BenchmarkResponse::badRequest('Constant not allowed');
    }
    $value = constant($name); // vuln-code-snippet safe-line php_vv_constant_lookup
    return BenchmarkResponse::ok((string)$value);
}
// vuln-code-snippet end php_vv_constant_lookup
