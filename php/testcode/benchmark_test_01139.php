<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01139(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $username = $req->param('username');
    $query = "SELECT * FROM accounts WHERE username = '" . $username . "'";
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
