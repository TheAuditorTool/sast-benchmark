<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00917(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = intval($req->param('id'));
    $query = "SELECT * FROM users WHERE id = " . $id;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
