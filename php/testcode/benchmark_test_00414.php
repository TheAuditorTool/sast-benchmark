<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00414(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $hashed = sha1($token);
    return BenchmarkResponse::json(['token_hash' => $hashed]);
}
