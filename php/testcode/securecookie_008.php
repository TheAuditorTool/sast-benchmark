<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_header_no_flags
function securecookie008(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: token=" . $token . "; Path=/"); // vuln-code-snippet vuln-line php_cookie_header_no_flags
    return BenchmarkResponse::ok('Token set');
}
// vuln-code-snippet end php_cookie_header_no_flags
