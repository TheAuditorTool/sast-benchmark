<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00655(BenchmarkRequest $req): BenchmarkResponse {
    $authHeader = $req->header('Authorization');
    if (!str_starts_with($authHeader, 'Bearer ')) {
        return BenchmarkResponse::error('unauthorized');
    }
    $token = substr($authHeader, 7);
    $user = validateBearerToken($token);
    if (!$user) {
        return BenchmarkResponse::error('invalid token');
    }
    return BenchmarkResponse::json(fetchUserData($user['id']));
}
