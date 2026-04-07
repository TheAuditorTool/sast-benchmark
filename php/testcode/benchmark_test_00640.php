<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00640(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('search');
    $stmt = $pdo->prepare("SELECT * FROM users WHERE name LIKE ?");
    $stmt->execute(['%' . $search . '%']);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
