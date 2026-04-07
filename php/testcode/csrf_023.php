<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_login_no_token
function csrf023(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->post('username');
    $password = $req->post('password');
    $user = authenticateUser($username, $password); // vuln-code-snippet vuln-line php_csrf_login_no_token
    if (!$user) {
        return BenchmarkResponse::badRequest('invalid credentials');
    }
    session_start();
    $_SESSION['user_id'] = $user['id'];
    return BenchmarkResponse::ok('logged in');
}
// vuln-code-snippet end php_csrf_login_no_token
