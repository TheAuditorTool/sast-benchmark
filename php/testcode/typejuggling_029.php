<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_strcmp_negated_array
function typejuggling029(BenchmarkRequest $req): BenchmarkResponse {
    $hash = $req->param('hash');
    $stored = 'secret123';
    if (!strcmp($hash, $stored)) { // vuln-code-snippet vuln-line php_tj_strcmp_negated_array // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('match');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_strcmp_negated_array
