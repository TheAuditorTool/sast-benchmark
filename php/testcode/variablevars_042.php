<?php
require_once __DIR__ . '/shared.php';

class Template {
    const VAR_NAME = 'title';
}

// vuln-code-snippet start php_vv_class_constant_var
function variablevars042(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    ${Template::VAR_NAME} = $val; // vuln-code-snippet safe-line php_vv_class_constant_var
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_class_constant_var
