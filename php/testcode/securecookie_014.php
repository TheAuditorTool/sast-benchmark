<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_options_array
function securecookie014(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('auth', $token, [
        'expires' => time() + 3600,
        'path' => '/',
        'secure' => true,
        'httponly' => true,
        'samesite' => 'Strict', // vuln-code-snippet safe-line php_cookie_options_array
    ]);
    return BenchmarkResponse::ok('Auth cookie set');
}
// vuln-code-snippet end php_cookie_options_array
