<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_strcmp_strict_zero
function typejuggling040(BenchmarkRequest $req): BenchmarkResponse {
    $a = $req->param('a');
    $b = $req->param('b');
    if (strcmp($a, $b) === 0) { // vuln-code-snippet safe-line php_tj_strcmp_strict_zero
        return BenchmarkResponse::ok('equal');
    }
    return BenchmarkResponse::ok('not equal');
}
// vuln-code-snippet end php_tj_strcmp_strict_zero
