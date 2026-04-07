<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_triple_protection
function securecookie044(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'samesite' => 'Strict', 'path' => '/']); // vuln-code-snippet safe-line php_cookie_triple_protection
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_triple_protection
