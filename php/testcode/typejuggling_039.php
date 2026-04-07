<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_intval_strict_compare
function typejuggling039(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('id');
    if (intval($input) === 1) { // vuln-code-snippet safe-line php_tj_intval_strict_compare
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::badRequest('not found');
}
// vuln-code-snippet end php_tj_intval_strict_compare
