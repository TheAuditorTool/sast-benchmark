<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_preg_alphanum
function headerinj017(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('token');
    if (!preg_match('/^[a-zA-Z0-9_-]+$/', $input)) {
        return BenchmarkResponse::badRequest('Invalid token format');
    }
    header("X-Auth-Token: " . $input); // vuln-code-snippet safe-line php_headerinj_preg_alphanum
    return BenchmarkResponse::ok('Authenticated');
}
// vuln-code-snippet end php_headerinj_preg_alphanum
