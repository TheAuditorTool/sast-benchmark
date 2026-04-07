<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_hash_equals_safe
function typejuggling033(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('token');
    $expected = getenv('SECRET_TOKEN');
    if (!hash_equals($expected, $input)) { // vuln-code-snippet safe-line php_tj_hash_equals_safe
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}
// vuln-code-snippet end php_tj_hash_equals_safe
