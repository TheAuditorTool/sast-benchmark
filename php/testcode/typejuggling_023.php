<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_strcmp_array_null
function typejuggling023(BenchmarkRequest $req): BenchmarkResponse {
    $hash = $req->param('hash');
    $expected = 'abc123';
    if (strcmp($hash, $expected) == 0) { // vuln-code-snippet vuln-line php_tj_strcmp_array_null // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('pass');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_strcmp_array_null
