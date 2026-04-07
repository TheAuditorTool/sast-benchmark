<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00873(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $sortCol = $req->param('sort');
    $allowed = ['id', 'name', 'email', 'created_at'];
    $safeSort = in_array($sortCol, $allowed, true) ? $sortCol : 'id';
    $query = "SELECT * FROM users ORDER BY " . $safeSort;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
