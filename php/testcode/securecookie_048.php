<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_sanitized_value_flags
function securecookie048(BenchmarkRequest $req): BenchmarkResponse {
    $theme = htmlspecialchars($req->param('theme'), ENT_QUOTES, 'UTF-8');
    setcookie('pref', $theme, ['secure' => true, 'httponly' => true, 'samesite' => 'Lax']); // vuln-code-snippet safe-line php_cookie_sanitized_value_flags
    return BenchmarkResponse::ok('pref saved');
}
// vuln-code-snippet end php_cookie_sanitized_value_flags
