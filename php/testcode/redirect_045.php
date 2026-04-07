<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_oidc_registered_uri
function redirect045(BenchmarkRequest $req): BenchmarkResponse {
    $redirectUri = $req->param('redirect_uri');
    $registered = ['https://app.example.com/callback'];
    if (!in_array($redirectUri, $registered, true)) {
        return BenchmarkResponse::badRequest('invalid uri');
    }
    header('Location: ' . $redirectUri); // vuln-code-snippet safe-line php_redirect_oidc_registered_uri
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_oidc_registered_uri
