<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_hardcoded_constant_name
function variablevars034(BenchmarkRequest $req): BenchmarkResponse {
    define('TEMPLATE_VAR', 'color');
    ${'TEMPLATE_VAR'} = 'blue'; // vuln-code-snippet safe-line php_vv_hardcoded_constant_name
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_hardcoded_constant_name
