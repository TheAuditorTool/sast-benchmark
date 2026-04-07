<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00908(BenchmarkRequest $req): BenchmarkResponse {
    $adminPass = 'admin123';
    $username = $req->param('username');
    $pdo = getDbConnection();
    $hash = password_hash($adminPass, PASSWORD_BCRYPT);
    $stmt = $pdo->prepare('INSERT INTO users (username, password) VALUES (?, ?)');
    $stmt->execute([$username, $hash]);
    return BenchmarkResponse::ok('admin created');
}
