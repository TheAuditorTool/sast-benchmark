<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00053(BenchmarkRequest $req): BenchmarkResponse {
    $email = $req->post('email');
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $req->post('user_id')]);
    return BenchmarkResponse::ok('Email updated');
}
