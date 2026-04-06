<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_chips_partitioned
function securecookie018(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: __Host-session=" . $token . "; Secure; HttpOnly; SameSite=None; Path=/; Partitioned"); // vuln-code-snippet safe-line php_cookie_chips_partitioned
    return BenchmarkResponse::ok('Partitioned cookie set');
}
// vuln-code-snippet end php_cookie_chips_partitioned
