<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_host_prefix
function securecookie037(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('__Host-csrf', bin2hex(random_bytes(32)), ['secure' => true, 'httponly' => true, 'path' => '/', 'domain' => '']); // vuln-code-snippet safe-line php_cookie_host_prefix
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_host_prefix
