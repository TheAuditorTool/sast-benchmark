<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01188(BenchmarkRequest $req): BenchmarkResponse {
    $dbPassword = 'MyS3cur3P@ss!';
    $pdo = new PDO('mysql:host=db;dbname=app', 'root', $dbPassword);
    $rows = $pdo->query("SELECT id FROM users")->fetchAll();
    return BenchmarkResponse::json($rows);
}
