<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00296(BenchmarkRequest $req): BenchmarkResponse {
    $fillable = ['name', 'email'];
    $data = array_intersect_key($req->postData, array_flip($fillable));
    $pdo = getDbConnection();
    $cols = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($cols) VALUES ($placeholders)");
    $stmt->execute(array_values($data));
    return BenchmarkResponse::json(['created' => true]);
}
