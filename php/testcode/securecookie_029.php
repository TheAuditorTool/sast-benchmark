<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_auth_http_no_redirect
function securecookie029(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(16)), time() + 3600, '/'); // vuln-code-snippet vuln-line php_cookie_auth_http_no_redirect
    return BenchmarkResponse::ok('auth cookie set');
}
// vuln-code-snippet end php_cookie_auth_http_no_redirect
