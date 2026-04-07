<?php
require_once __DIR__ . '/shared.php';

function sanitizeForQuery(string $id): int {
    return intval($id);
}

function benchmarkTest00118(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $safeId = sanitizeForQuery($id);
    $query = "SELECT * FROM users WHERE id = " . $safeId;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
