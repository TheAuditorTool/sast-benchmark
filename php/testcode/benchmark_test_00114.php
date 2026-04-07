<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00114(BenchmarkRequest $req): BenchmarkResponse {
    $auth_level = 'none';
    $secret_key = getenv('APP_SECRET');
    $key = $req->param('key');
    $val = $req->param('val');
    $$key = $val;
    return BenchmarkResponse::json(['auth' => $auth_level]);
}
