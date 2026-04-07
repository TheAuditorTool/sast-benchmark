<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00576(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT * FROM users u WHERE u.id = :id");
    $stmt->execute(['id' => $id]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
