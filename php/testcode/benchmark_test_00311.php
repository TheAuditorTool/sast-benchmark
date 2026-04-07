<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00311(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', 's3cr3t');
    $stmt = $pdo->query('SELECT COUNT(*) FROM users');
    $count = $stmt->fetchColumn();
    return BenchmarkResponse::ok((string)$count);
}
