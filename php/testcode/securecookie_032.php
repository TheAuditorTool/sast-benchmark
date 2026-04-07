<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_no_samesite_secure
function securecookie032(BenchmarkRequest $req): BenchmarkResponse {
    session_set_cookie_params(['lifetime' => 0, 'path' => '/']); // vuln-code-snippet vuln-line php_cookie_session_no_samesite_secure
    session_start();
    return BenchmarkResponse::ok('session started');
}
// vuln-code-snippet end php_cookie_session_no_samesite_secure
