<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01145(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $ids = $req->param('ids');
    $result = $pdo->query("SELECT * FROM orders WHERE id IN (" . $ids . ")");
    return BenchmarkResponse::json($result->fetchAll());
}
