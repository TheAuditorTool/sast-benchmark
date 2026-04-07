<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_jwt_no_httponly
function securecookie022(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = 'eyJhbGc...fake.jwt.token';
    setcookie('jwt', $jwt, time() + 3600, '/', '', true, false); // vuln-code-snippet vuln-line php_cookie_jwt_no_httponly
    return BenchmarkResponse::ok('jwt set');
}
// vuln-code-snippet end php_cookie_jwt_no_httponly
