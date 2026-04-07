<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_token_rotate_privileged
function securecookie040(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    if ($req->param('action') === 'elevate') {
        session_regenerate_id(true); // vuln-code-snippet safe-line php_cookie_token_rotate_privileged
        $_SESSION['role'] = 'admin';
    }
    return BenchmarkResponse::ok('elevated');
}
// vuln-code-snippet end php_cookie_token_rotate_privileged
