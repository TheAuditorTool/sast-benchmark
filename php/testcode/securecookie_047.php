<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_short_maxage_rotation
function securecookie047(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', bin2hex(random_bytes(32)), ['expires' => time() + 900, 'secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line php_cookie_short_maxage_rotation
    return BenchmarkResponse::ok('cookie set');
}
// vuln-code-snippet end php_cookie_short_maxage_rotation
