<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00313(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $tableName = $req->param('table');
    $query = "SELECT * FROM " . $tableName;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
