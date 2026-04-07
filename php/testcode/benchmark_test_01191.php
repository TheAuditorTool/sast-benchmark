<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01191(BenchmarkRequest $req): BenchmarkResponse {
    $host = getenv('DB_HOST');
    $user = getenv('DB_USER');
    $pass = getenv('DB_PASS');
    $pdo = new PDO("mysql:host=$host;dbname=app", $user, $pass);
    $rows = $pdo->query("SELECT id FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
