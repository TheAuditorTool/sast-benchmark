<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_jwt_bearer_header
function securecookie039(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = 'eyJhbGc...fake';
    header('Authorization: Bearer ' . $jwt); // vuln-code-snippet safe-line php_cookie_jwt_bearer_header
    return BenchmarkResponse::ok('jwt in header');
}
// vuln-code-snippet end php_cookie_jwt_bearer_header
