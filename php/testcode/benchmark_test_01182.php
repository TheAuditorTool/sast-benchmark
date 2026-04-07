<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01182(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $uid = $req->post('user_id');
    $stmt = $pdo->prepare("DELETE FROM accounts WHERE id = ?");
    $stmt->execute([$uid]);
    return BenchmarkResponse::ok('account removed');
}
