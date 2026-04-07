<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00104(BenchmarkRequest $req): BenchmarkResponse {
    $redirectUri = $req->param('redirect_uri');
    $registered = ['https://app.example.com/callback'];
    if (!in_array($redirectUri, $registered, true)) {
        return BenchmarkResponse::badRequest('invalid uri');
    }
    header('Location: ' . $redirectUri);
    return BenchmarkResponse::ok('');
}
