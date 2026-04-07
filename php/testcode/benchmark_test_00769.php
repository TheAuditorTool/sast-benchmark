<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00769(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $safeId = $pdo->quote($id);
    $query = "SELECT * FROM users WHERE id = " . $safeId;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
