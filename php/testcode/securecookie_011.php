<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_name_insecure
function securecookie011(BenchmarkRequest $req): BenchmarkResponse {
    session_name('MYAPP_SID');
    session_start(); // vuln-code-snippet vuln-line php_cookie_session_name_insecure
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Logged in');
}
// vuln-code-snippet end php_cookie_session_name_insecure
