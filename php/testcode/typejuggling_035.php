<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_int_cast_strict
function typejuggling035(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if ((int)$id === 42) { // vuln-code-snippet safe-line php_tj_int_cast_strict
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::badRequest('not found');
}
// vuln-code-snippet end php_tj_int_cast_strict
