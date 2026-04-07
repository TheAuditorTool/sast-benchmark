<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00127(BenchmarkRequest $req): BenchmarkResponse {
    $secret = "my-super-secret-jwt-key-2024";
    $payload = json_encode(['user' => $req->param('user'), 'exp' => time() + 3600]);
    $signature = hash_hmac('sha256', $payload, $secret);
    return BenchmarkResponse::json(['token' => base64_encode($payload) . '.' . $signature]);
}
