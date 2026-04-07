<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00187(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $cols = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($cols) VALUES ($placeholders)");
    $stmt->execute(array_values($data));
    return BenchmarkResponse::json(['created' => true]);
}
