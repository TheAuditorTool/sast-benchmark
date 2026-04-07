<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_settype_then_strict
function typejuggling044(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('n');
    settype($input, 'integer'); // vuln-code-snippet safe-line php_tj_settype_then_strict
    if ($input === 42) {
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
// vuln-code-snippet end php_tj_settype_then_strict
