<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_integer_loop_counter
function variablevars035(BenchmarkRequest $req): BenchmarkResponse {
    for ($i = 0; $i < 3; $i++) {
        $$i = $i * 2; // vuln-code-snippet safe-line php_vv_integer_loop_counter
    }
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_vv_integer_loop_counter
