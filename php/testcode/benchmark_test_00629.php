<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00629(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $userId = (int) ($_SESSION['user_id'] ?? 0);
    $prefs = $_SESSION['preferences'] ?? [];
    return BenchmarkResponse::json(['user' => $userId, 'prefs' => $prefs]);
}
