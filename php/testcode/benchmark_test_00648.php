<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00648(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $state = $req->param('state');
    $code = $req->param('code');
    if (!hash_equals($_SESSION['oauth_state'] ?? '', $state)) {
        return BenchmarkResponse::badRequest('state mismatch');
    }
    $codeVerifier = $_SESSION['pkce_verifier'] ?? '';
    $tokens = exchangeCodeForTokens($code, $codeVerifier);
    return BenchmarkResponse::json(['access_token' => $tokens['access_token']]);
}
