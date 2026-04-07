<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00057(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = sprintf("SELECT * FROM users WHERE id = '%s'", $id);
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
