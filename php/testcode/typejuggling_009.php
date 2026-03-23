<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_strcmp_null
function typejuggling009(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('password');
    $secret = 'supersecretpassword';
    if (strcmp($input, $secret) == 0) { // vuln-code-snippet vuln-line php_tj_strcmp_null
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}
// vuln-code-snippet end php_tj_strcmp_null
