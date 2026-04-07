<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01057(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $sortCol = $req->param('sort');
    $query = "SELECT * FROM users ORDER BY " . $sortCol;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
