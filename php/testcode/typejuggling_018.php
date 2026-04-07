<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_in_array_coerce_int
function typejuggling018(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    if (in_array($role, [1, 2, 3])) { // vuln-code-snippet vuln-line php_tj_in_array_coerce_int // Legacy PHP 7.x pattern
        return BenchmarkResponse::ok('admin');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_in_array_coerce_int
