<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_oauth_pkce_state
function csrf033(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $state = $req->param('state');
    $code = $req->param('code');
    if (!hash_equals($_SESSION['oauth_state'] ?? '', $state)) { // vuln-code-snippet safe-line php_csrf_oauth_pkce_state
        return BenchmarkResponse::badRequest('state mismatch');
    }
    $codeVerifier = $_SESSION['pkce_verifier'] ?? '';
    $tokens = exchangeCodeForTokens($code, $codeVerifier);
    return BenchmarkResponse::json(['access_token' => $tokens['access_token']]);
}
// vuln-code-snippet end php_csrf_oauth_pkce_state
