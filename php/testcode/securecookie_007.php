<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_params_no_secure
function securecookie007(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['httponly' => true, 'samesite' => 'Lax']);
    session_start(); // vuln-code-snippet vuln-line php_cookie_params_no_secure
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_params_no_secure
