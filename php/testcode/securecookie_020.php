<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_samesite_none_no_secure
function securecookie020(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', session_id(), ['samesite' => 'None', 'httponly' => true]); // vuln-code-snippet vuln-line php_cookie_samesite_none_no_secure
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_samesite_none_no_secure
