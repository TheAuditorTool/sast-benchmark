<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00590(BenchmarkRequest $req): BenchmarkResponse {
    $pass = getenv('DB_PASSWORD') ?: throw new RuntimeException('DB_PASSWORD not set');
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
