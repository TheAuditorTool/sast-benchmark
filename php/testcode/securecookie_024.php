<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_weak_session_id
function securecookie024(BenchmarkRequest $req): BenchmarkResponse {
    session_id(md5((string) time())); // vuln-code-snippet vuln-line php_cookie_weak_session_id
    session_start();
    return BenchmarkResponse::ok('session started');
}
// vuln-code-snippet end php_cookie_weak_session_id
