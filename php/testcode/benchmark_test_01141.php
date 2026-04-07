<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01141(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $category = $req->param('category');
    $order = $req->param('order');
    $sql = sprintf("SELECT * FROM products WHERE category = '%s' ORDER BY %s", $category, $order);
    $stmt = $pdo->query($sql);
    return BenchmarkResponse::json($stmt->fetchAll());
}
