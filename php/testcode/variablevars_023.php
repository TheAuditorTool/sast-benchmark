<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_post_loop_global
function variablevars023(BenchmarkRequest $req): BenchmarkResponse {
    foreach ($_POST as $k => $v) {
        $$k = $v; // vuln-code-snippet vuln-line php_vv_post_loop_global
    }
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_vv_post_loop_global
