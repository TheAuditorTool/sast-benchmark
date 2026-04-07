<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_chips_partitioned_safe
function securecookie045(BenchmarkRequest $req): BenchmarkResponse {
    header('Set-Cookie: __Host-session=' . bin2hex(random_bytes(32)) . '; Secure; HttpOnly; SameSite=None; Path=/; Partitioned'); // vuln-code-snippet safe-line php_cookie_chips_partitioned_safe
    return BenchmarkResponse::ok('partitioned cookie set');
}
// vuln-code-snippet end php_cookie_chips_partitioned_safe
