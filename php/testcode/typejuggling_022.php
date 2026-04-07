<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_truthy_string_compare
function typejuggling022(BenchmarkRequest $req): BenchmarkResponse {
    $user = $req->param('user');
    if ($user == true) { // vuln-code-snippet vuln-line php_tj_truthy_string_compare
        return BenchmarkResponse::ok('logged in');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_truthy_string_compare
