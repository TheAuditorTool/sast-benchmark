<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_header_all_flags
function securecookie015(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: token=" . $token . "; Secure; HttpOnly; SameSite=Strict; Path=/"); // vuln-code-snippet safe-line php_cookie_header_all_flags
    return BenchmarkResponse::ok('Token set');
}
// vuln-code-snippet end php_cookie_header_all_flags
