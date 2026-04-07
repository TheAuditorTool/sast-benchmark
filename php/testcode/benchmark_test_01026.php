<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01026(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = "SELECT name, email FROM users WHERE id=" . $id;
    $result = $pdo->query($query);
    $row = $result->fetch();
    return BenchmarkResponse::json($row ?: []);
}
