<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_js_accessible
function securecookie005(BenchmarkRequest $req): BenchmarkResponse {
    $val = bin2hex(random_bytes(16));
    setcookie("auth", $val, time() + 3600, "/", "", false, false); // vuln-code-snippet vuln-line php_cookie_js_accessible
    return BenchmarkResponse::ok('Auth cookie set');
}
// vuln-code-snippet end php_cookie_js_accessible
