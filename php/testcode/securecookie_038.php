<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_regenerate_after_login
function securecookie038(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    session_regenerate_id(true); // vuln-code-snippet safe-line php_cookie_regenerate_after_login
    $_SESSION['user_id'] = 1;
    return BenchmarkResponse::ok('logged in');
}
// vuln-code-snippet end php_cookie_regenerate_after_login
