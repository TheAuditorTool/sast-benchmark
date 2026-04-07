<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_oauth_secret
function hardcodedcreds020(BenchmarkRequest $req): BenchmarkResponse {
    $config = ['client_id' => 'myapp', 'client_secret' => 'oauth_secret_abc123']; // vuln-code-snippet vuln-line php_hardcoded_oauth_secret
    $code = $req->param('code');
    $payload = http_build_query([
        'grant_type' => 'authorization_code',
        'code' => $code,
        'client_id' => $config['client_id'],
        'client_secret' => $config['client_secret'],
    ]);
    return BenchmarkResponse::ok($payload);
}
// vuln-code-snippet end php_hardcoded_oauth_secret
