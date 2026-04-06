<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_ini_secure
function securecookie016(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.cookie_secure', '1');
    ini_set('session.cookie_httponly', '1');
    ini_set('session.cookie_samesite', 'Lax'); // vuln-code-snippet safe-line php_cookie_ini_secure
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_ini_secure
