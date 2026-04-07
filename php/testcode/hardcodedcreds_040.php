<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_pkce_no_secret
function hardcodedcreds040(BenchmarkRequest $req): BenchmarkResponse {
    $config = ['client_id' => 'public-spa-client', 'redirect_uri' => 'https://app.example.com/callback']; // vuln-code-snippet safe-line php_hardcoded_pkce_no_secret
    $codeVerifier = bin2hex(random_bytes(32));
    $codeChallenge = rtrim(strtr(base64_encode(hash('sha256', $codeVerifier, true)), '+/', '-_'), '=');
    $url = 'https://auth.example.com/authorize?' . http_build_query([
        'client_id' => $config['client_id'],
        'code_challenge' => $codeChallenge,
        'code_challenge_method' => 'S256',
        'redirect_uri' => $config['redirect_uri'],
    ]);
    return BenchmarkResponse::redirect($url);
}
// vuln-code-snippet end php_hardcoded_pkce_no_secret
