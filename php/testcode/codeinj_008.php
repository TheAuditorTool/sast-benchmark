<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_closure
function codeinj_closure(BenchmarkRequest $req): BenchmarkResponse {
    $value = $req->param('value');
    $fn = function ($x) { return $x * 2; }; // vuln-code-snippet safe-line php_codeinj_closure
    $result = $fn((int) $value);
    return BenchmarkResponse::ok("Result: " . $result);
}
// vuln-code-snippet end php_codeinj_closure
