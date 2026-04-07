<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00535(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $status = $req->param('status');
    $stmt = $pdo->prepare("SELECT * FROM orders WHERE status = ?");
    $stmt->execute([$status]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
