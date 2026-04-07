<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_session_in_url
function securecookie021(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $url = $req->param('url') . '?' . SID; // vuln-code-snippet vuln-line php_cookie_session_in_url
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_cookie_session_in_url
