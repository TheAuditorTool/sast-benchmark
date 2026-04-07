<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_strict_zero_compare
function typejuggling032(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    if ($token === '0') { // vuln-code-snippet safe-line php_tj_strict_zero_compare
        return BenchmarkResponse::ok('zero token');
    }
    return BenchmarkResponse::ok('other token');
}
// vuln-code-snippet end php_tj_strict_zero_compare
