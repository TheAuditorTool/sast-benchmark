<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00263(BenchmarkRequest $req): BenchmarkResponse {
    $config = ['client_id' => 'myapp', 'client_secret' => 'oauth_secret_abc123'];
    $code = $req->param('code');
    $payload = http_build_query([
        'grant_type' => 'authorization_code',
        'code' => $code,
        'client_id' => $config['client_id'],
        'client_secret' => $config['client_secret'],
    ]);
    return BenchmarkResponse::ok($payload);
}
