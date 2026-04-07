<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00744(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
