<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_hsts_plus_secure
function securecookie041(BenchmarkRequest $req): BenchmarkResponse {
    header('Strict-Transport-Security: max-age=63072000; includeSubDomains');
    setcookie('auth', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line php_cookie_hsts_plus_secure
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_hsts_plus_secure
