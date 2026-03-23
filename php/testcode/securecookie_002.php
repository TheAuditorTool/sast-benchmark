<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_full_flags
function securecookie002(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie("session", $token, ['secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line php_cookie_full_flags
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_full_flags
