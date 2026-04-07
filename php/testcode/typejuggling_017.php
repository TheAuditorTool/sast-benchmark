<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_zero_equals_string
function typejuggling017(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    if ($token == 0) { // vuln-code-snippet vuln-line php_tj_zero_equals_string // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_zero_equals_string
