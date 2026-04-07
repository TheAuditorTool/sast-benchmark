<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00990(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = $req->header('Authorization');
    $parts = explode('.', $jwt);
    $payload = json_decode(base64_decode($parts[1] ?? ''), true);
    $obj = unserialize(base64_decode($payload['data'] ?? ''));
    return BenchmarkResponse::ok('processed');
}
