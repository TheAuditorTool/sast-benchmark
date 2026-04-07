<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00435(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $ids = explode(',', $req->param('ids'));
    $placeholders = implode(',', array_fill(0, count($ids), '?'));
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id IN (" . $placeholders . ")");
    $stmt->execute($ids);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
