<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_hmac_signed
function securecookie043(BenchmarkRequest $req): BenchmarkResponse {
    $data = 'user_id=1';
    $sig = hash_hmac('sha256', $data, getenv('COOKIE_SECRET'));
    setcookie('auth', $data . '.' . $sig, ['secure' => true, 'httponly' => true]); // vuln-code-snippet safe-line php_cookie_hmac_signed
    return BenchmarkResponse::ok('signed cookie set');
}
// vuln-code-snippet end php_cookie_hmac_signed
