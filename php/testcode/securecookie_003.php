<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_default
function securecookie003(BenchmarkRequest $req): BenchmarkResponse {
    session_start(); // vuln-code-snippet vuln-line php_cookie_session_default
    $_SESSION['user'] = $req->param('user');
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_session_default
