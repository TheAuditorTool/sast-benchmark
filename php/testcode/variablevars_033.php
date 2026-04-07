<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_global_keyword_loop
function variablevars033(BenchmarkRequest $req): BenchmarkResponse {
    $vars = $_POST;
    foreach ($vars as $k => $v) {
        global $$k;
        $$k = $v; // vuln-code-snippet vuln-line php_vv_global_keyword_loop
    }
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_vv_global_keyword_loop
