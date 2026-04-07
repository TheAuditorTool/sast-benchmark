<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00125(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT age FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $age = $stmt->fetchColumn();
    $safeAge = intval($age);
    $result = $pdo->query("SELECT * FROM stats WHERE age = " . $safeAge);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
