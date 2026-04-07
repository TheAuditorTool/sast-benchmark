<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01153(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $uid = $req->cookie('user_id');
    $role = $req->param('role');
    $stmt = $pdo->prepare("UPDATE users SET role = ? WHERE id = ?");
    $stmt->execute([$role, $uid]);
    return BenchmarkResponse::ok('updated');
}
