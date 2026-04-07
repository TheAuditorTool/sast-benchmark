<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00258(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $decoded = json_decode(base64_decode($token), true);
    if (!is_array($decoded)) {
        return BenchmarkResponse::badRequest('invalid token');
    }
    return BenchmarkResponse::json($decoded);
}
