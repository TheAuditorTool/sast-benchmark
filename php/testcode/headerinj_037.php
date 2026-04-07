<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_int_cast_length
function headerinj037(BenchmarkRequest $req): BenchmarkResponse {
    $size = $req->param('size');
    header('Content-Length: ' . (int)$size); // vuln-code-snippet safe-line php_headerinj_int_cast_length
    return BenchmarkResponse::ok(str_repeat('x', (int)$size));
}
// vuln-code-snippet end php_headerinj_int_cast_length
