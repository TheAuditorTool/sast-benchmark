<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01149(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $username = $req->param('username');
    $stmt = $pdo->prepare("SELECT * FROM accounts WHERE username = ?");
    $stmt->execute([$username]);
    return BenchmarkResponse::json($stmt->fetchAll());
}
