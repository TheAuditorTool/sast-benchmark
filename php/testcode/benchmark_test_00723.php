<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00723(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('search');
    $query = "SELECT * FROM users WHERE name LIKE '%" . $search . "%'";
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
