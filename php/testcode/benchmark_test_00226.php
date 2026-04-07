<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00226(BenchmarkRequest $req): BenchmarkResponse {
    $config = ['client_id' => 'public-spa-client', 'redirect_uri' => 'https://app.example.com/callback'];
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
