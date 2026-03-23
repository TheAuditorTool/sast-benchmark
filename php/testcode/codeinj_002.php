<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_eval_hardcoded
function codeinj_eval_hardcoded(BenchmarkRequest $req): BenchmarkResponse {
    $result = eval('return 2 + 2;'); // vuln-code-snippet safe-line php_codeinj_eval_hardcoded
    return BenchmarkResponse::ok("Result: " . $result);
}
// vuln-code-snippet end php_codeinj_eval_hardcoded
