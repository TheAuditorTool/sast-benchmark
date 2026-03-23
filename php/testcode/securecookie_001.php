<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_no_flags
function securecookie001(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie("session", $token); // vuln-code-snippet vuln-line php_cookie_no_flags
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_no_flags
