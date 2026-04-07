<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_no_secure_httponly
function securecookie019(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(16));
    setcookie('auth', $token, 0, '/', '', false, false); // vuln-code-snippet vuln-line php_cookie_no_secure_httponly
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_no_secure_httponly
