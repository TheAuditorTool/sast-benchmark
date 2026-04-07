<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01157(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $category = $req->param('category');
    $stmt = $pdo->prepare("SELECT * FROM products WHERE category = ?");
    $stmt->execute([$category]);
    return BenchmarkResponse::json($stmt->fetchAll());
}
