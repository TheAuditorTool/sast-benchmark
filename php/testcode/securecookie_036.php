<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_secure_prefix
function securecookie036(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('__Secure-session', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'path' => '/']); // vuln-code-snippet safe-line php_cookie_secure_prefix
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_secure_prefix
