<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_oauth_uri_unvalidated
function redirect022(BenchmarkRequest $req): BenchmarkResponse {
    $redirectUri = $req->param('redirect_uri');
    header('Location: ' . $redirectUri); // vuln-code-snippet vuln-line php_redirect_oauth_uri_unvalidated
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_oauth_uri_unvalidated
