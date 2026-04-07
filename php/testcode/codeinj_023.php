<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_assert_string
function codeinj023(BenchmarkRequest $req): BenchmarkResponse {
    $expr = $req->param('expr');
    // Legacy PHP 7.x pattern
    $result = assert($expr); // vuln-code-snippet vuln-line php_codeinj_assert_string
    return BenchmarkResponse::ok($result ? 'true' : 'false');
}
// vuln-code-snippet end php_codeinj_assert_string
