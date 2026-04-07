<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_assert_bool_expr
function codeinj048(BenchmarkRequest $req): BenchmarkResponse {
    $str = $req->param('value');
    assert(strlen($str) > 0); // vuln-code-snippet safe-line php_codeinj_assert_bool_expr
    return BenchmarkResponse::ok('non-empty: ' . strlen($str));
}
// vuln-code-snippet end php_codeinj_assert_bool_expr
