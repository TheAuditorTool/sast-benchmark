<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_params_array_full
function securecookie013(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params([
        'lifetime' => 3600,
        'path' => '/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Lax', // vuln-code-snippet safe-line php_cookie_params_array_full
    ]);
    session_start();
    $_SESSION['user'] = $req->post('username');
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_params_array_full
