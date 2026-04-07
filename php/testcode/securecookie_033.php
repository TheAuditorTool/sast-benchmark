<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_path_root_no_domain
function securecookie033(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('track', 'value', time() + 3600, '/'); // vuln-code-snippet vuln-line php_cookie_path_root_no_domain
    return BenchmarkResponse::ok('tracked');
}
// vuln-code-snippet end php_cookie_path_root_no_domain
