<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01132(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $columns = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($columns) VALUES ($placeholders)");
    $stmt->execute(array_values($data));
    return BenchmarkResponse::ok('User created');
}
