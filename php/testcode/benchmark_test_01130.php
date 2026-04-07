<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01130(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = "SELECT * FROM users WHERE id = " . $id;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
