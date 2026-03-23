<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_safe
function securecookie004(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.cookie_secure', '1');
    ini_set('session.cookie_httponly', '1');
    ini_set('session.cookie_samesite', 'Strict'); // vuln-code-snippet safe-line php_cookie_session_safe
    session_start();
    $_SESSION['user'] = $req->param('user');
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_session_safe
