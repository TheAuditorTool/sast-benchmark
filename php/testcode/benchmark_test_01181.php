<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01181(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $uid = $req->post('user_id');
    $email = $req->post('email');
    $stmt = $pdo->prepare("UPDATE users SET email = ? WHERE id = ?");
    $stmt->execute([$email, $uid]);
    return BenchmarkResponse::ok('email updated');
}
