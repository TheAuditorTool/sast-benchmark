<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_partitioned
function securecookie006(BenchmarkRequest $req): BenchmarkResponse {
    $val = bin2hex(random_bytes(16));
    setcookie("prefs", $val, ['expires' => time() + 3600, 'path' => '/', 'secure' => true, 'httponly' => true, 'samesite' => 'Lax']); // vuln-code-snippet safe-line php_cookie_partitioned
    return BenchmarkResponse::ok('Preferences cookie set');
}
// vuln-code-snippet end php_cookie_partitioned
