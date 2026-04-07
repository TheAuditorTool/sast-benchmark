<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00184(BenchmarkRequest $req): BenchmarkResponse {
    $body = json_decode($req->bodyStr(), true);
    $userId = (int) ($body['user_id'] ?? 0);
    $role = $body['role'] ?? 'viewer';
    updateUserRole($userId, $role);
    return BenchmarkResponse::json(['updated' => true]);
}
