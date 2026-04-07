<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_no_secure_ini
function securecookie028(BenchmarkRequest $req): BenchmarkResponse {
    session_start(); // vuln-code-snippet vuln-line php_cookie_session_no_secure_ini
    $_SESSION['user'] = $req->param('user');
    return BenchmarkResponse::ok('session started');
}
// vuln-code-snippet end php_cookie_session_no_secure_ini
