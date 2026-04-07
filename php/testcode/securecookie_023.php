<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_fixation
function securecookie023(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $_SESSION['user_id'] = 1; // vuln-code-snippet vuln-line php_cookie_session_fixation
    return BenchmarkResponse::ok('logged in');
}
// vuln-code-snippet end php_cookie_session_fixation
