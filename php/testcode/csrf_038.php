<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_secure_prefix_cookie
function csrf038(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $token = bin2hex(random_bytes(32));
    $_SESSION['csrf_token'] = $token;
    setcookie('__Secure-csrf', $token, [ // vuln-code-snippet safe-line php_csrf_secure_prefix_cookie
        'secure'   => true,
        'httponly' => false,
        'samesite' => 'Strict',
        'path'     => '/',
    ]);
    return BenchmarkResponse::ok('token set');
}
// vuln-code-snippet end php_csrf_secure_prefix_cookie
