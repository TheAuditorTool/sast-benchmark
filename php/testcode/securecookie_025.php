<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_domain_too_broad
function securecookie025(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(16)), time() + 3600, '/', '.example.com', true, true); // vuln-code-snippet vuln-line php_cookie_domain_too_broad
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_domain_too_broad
