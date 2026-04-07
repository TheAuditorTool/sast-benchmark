<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_csrf_no_secure
function securecookie030(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('csrf', $token, time() + 3600, '/', '', false, false); // vuln-code-snippet vuln-line php_cookie_csrf_no_secure
    return BenchmarkResponse::ok('csrf token set');
}
// vuln-code-snippet end php_cookie_csrf_no_secure
