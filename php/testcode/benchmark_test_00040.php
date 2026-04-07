<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00040(BenchmarkRequest $req): BenchmarkResponse {
    $dsn = 'mysql:host=localhost;dbname=app';
    $pdo = new PDO($dsn, getenv('DB_USER'), getenv('DB_PASS'));
    $rows = $pdo->query("SELECT id, name FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
