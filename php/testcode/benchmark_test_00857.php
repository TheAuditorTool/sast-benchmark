<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00857(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $tableName = $req->param('table');
    $allowed = ['users', 'posts', 'comments'];
    if (!in_array($tableName, $allowed, true)) {
        return BenchmarkResponse::badRequest("Invalid table name");
    }
    $query = "SELECT * FROM " . $tableName;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
