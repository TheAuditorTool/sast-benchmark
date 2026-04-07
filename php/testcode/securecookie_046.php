<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_strict_mode_ini
function securecookie046(BenchmarkRequest $req): BenchmarkResponse {
    ini_set('session.use_strict_mode', '1');
    ini_set('session.cookie_secure', '1');
    session_start(); // vuln-code-snippet safe-line php_cookie_strict_mode_ini
    return BenchmarkResponse::ok('session started');
}
// vuln-code-snippet end php_cookie_strict_mode_ini
