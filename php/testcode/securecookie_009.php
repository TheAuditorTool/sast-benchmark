<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_samesite_none_insecure
function securecookie009(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    setcookie('session', $token, [
        'expires' => time() + 3600,
        'path' => '/',
        'samesite' => 'None',
        'secure' => false, // vuln-code-snippet vuln-line php_cookie_samesite_none_insecure
        'httponly' => true,
    ]);
    return BenchmarkResponse::ok('Session started');
}
// vuln-code-snippet end php_cookie_samesite_none_insecure
