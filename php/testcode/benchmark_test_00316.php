<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00316(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $dql = "SELECT * FROM users u WHERE u.id = " . $id;
    $result = $pdo->query($dql);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
