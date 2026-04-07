<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00887(BenchmarkRequest $req): BenchmarkResponse {
    $redirectUri = $req->param('redirect_uri');
    header('Location: ' . $redirectUri);
    return BenchmarkResponse::ok('');
}
