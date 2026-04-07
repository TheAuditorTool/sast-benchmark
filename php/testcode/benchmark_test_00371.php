<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00371(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $userId = $_SESSION['user_id'] ?? null;
    if (!$userId) {
        return BenchmarkResponse::error('not authenticated', 401);
    }
    return BenchmarkResponse::json(['user_id' => $userId]);
}
