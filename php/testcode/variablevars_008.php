<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_curly_syntax
function variablevars008(BenchmarkRequest $req): BenchmarkResponse {
    $config_db = 'production';
    $config_debug = false;
    $name = $req->param('name');
    $value = $req->param('value');
    ${$name} = $value; // vuln-code-snippet vuln-line php_vv_curly_syntax
    return BenchmarkResponse::json(['db' => $config_db, 'debug' => $config_debug]);
}
// vuln-code-snippet end php_vv_curly_syntax
