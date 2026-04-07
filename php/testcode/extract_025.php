<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_loop_variable_var
function extract025(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    foreach ($_POST as $k => $v) {
        $$k = $v; // vuln-code-snippet vuln-line php_extract_loop_variable_var
    }
    return BenchmarkResponse::ok('prefs set admin=' . ($isAdmin ? '1' : '0'));
}
// vuln-code-snippet end php_extract_loop_variable_var
