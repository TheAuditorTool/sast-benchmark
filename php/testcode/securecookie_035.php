<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_options_array_full
function securecookie035(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(32)), ['expires' => time() + 3600, 'path' => '/', 'secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line php_cookie_options_array_full
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_options_array_full
