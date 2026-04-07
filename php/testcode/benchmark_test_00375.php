<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00375(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT username FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $username = $stmt->fetchColumn();
    $result = $pdo->query("SELECT * FROM logs WHERE actor = '" . $username . "'");
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
