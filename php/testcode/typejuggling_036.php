<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_ctype_digit_strict
function typejuggling036(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    if (ctype_digit($input) && (int)$input === 100) { // vuln-code-snippet safe-line php_tj_ctype_digit_strict
        return BenchmarkResponse::ok('match');
    }
    return BenchmarkResponse::badRequest('not match');
}
// vuln-code-snippet end php_tj_ctype_digit_strict
