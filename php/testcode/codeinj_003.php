<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_assert
function codeinj_assert(BenchmarkRequest $req): BenchmarkResponse {
    $expr = $req->param('expr');
    assert($expr); // vuln-code-snippet vuln-line php_codeinj_assert
    return BenchmarkResponse::ok("Assertion passed");
}
// vuln-code-snippet end php_codeinj_assert
