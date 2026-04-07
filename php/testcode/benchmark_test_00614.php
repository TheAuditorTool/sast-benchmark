<?php
require_once __DIR__ . '/shared.php';

function buildQuery(string $id): string {
    return "SELECT * FROM users WHERE id = " . $id;
}

function benchmarkTest00614(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = buildQuery($id);
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
