<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_loose_compare_auth
function typejuggling001(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $expected = '0e462097431906509019562988736854';
    if ($token == $expected) { // vuln-code-snippet vuln-line php_tj_loose_compare_auth
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}
// vuln-code-snippet end php_tj_loose_compare_auth
