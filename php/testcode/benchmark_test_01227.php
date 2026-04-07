<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01227(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("INSERT INTO password_resets (email, token) VALUES (?, ?)");
    $stmt->execute([$req->post('email'), $token]);
    return BenchmarkResponse::ok('reset link sent');
}
