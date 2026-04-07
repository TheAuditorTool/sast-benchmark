<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_full_flags
function securecookie034(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line php_cookie_session_full_flags
    session_start();
    return BenchmarkResponse::ok('session started');
}
// vuln-code-snippet end php_cookie_session_full_flags
