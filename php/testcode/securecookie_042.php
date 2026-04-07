<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_random_bytes_value
function securecookie042(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('session', $token, ['secure' => true, 'httponly' => true, 'samesite' => 'Lax']); // vuln-code-snippet safe-line php_cookie_random_bytes_value
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_random_bytes_value
