<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00214(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $status = $req->param('status');
    $raw = "status = '" . $status . "'";
    $result = $pdo->query("SELECT * FROM orders WHERE " . $raw);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
