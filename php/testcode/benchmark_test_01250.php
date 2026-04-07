<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01250(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hash = password_hash($password, PASSWORD_BCRYPT);
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET password_hash = ? WHERE id = ?");
    $stmt->execute([$hash, $req->post('user_id')]);
    return BenchmarkResponse::ok('password updated');
}
