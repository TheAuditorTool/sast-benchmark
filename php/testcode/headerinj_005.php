<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_setcookie_name
function headerinj005(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $value = 'active';
    setcookie($name, $value); // vuln-code-snippet vuln-line php_headerinj_setcookie_name
    return BenchmarkResponse::ok('Cookie set');
}
// vuln-code-snippet end php_headerinj_setcookie_name
