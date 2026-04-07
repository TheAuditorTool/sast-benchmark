<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00246(BenchmarkRequest $req): BenchmarkResponse {
    $dsn = 'mysql:host=localhost;dbname=app';
    $pdo = new PDO($dsn, "root", "s3cr3t_p4ss");
    $rows = $pdo->query("SELECT id, name FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
