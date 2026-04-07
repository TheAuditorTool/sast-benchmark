<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00518(BenchmarkRequest $req): BenchmarkResponse {
    $sessionToken = $_SESSION['csrf_token'] ?? '';
    if (!hash_equals($sessionToken, $req->post('csrf_token'))) {
        return BenchmarkResponse::badRequest('Invalid CSRF token');
    }
    $email = $req->post('email');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $req->post('user_id')]);
    return BenchmarkResponse::ok('Email updated');
}
