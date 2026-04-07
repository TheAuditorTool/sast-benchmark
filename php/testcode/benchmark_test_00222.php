<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00222(BenchmarkRequest $req): BenchmarkResponse {
    define('DB_PASSWORD', 'hunter2');
    $pdo = new PDO(
        'mysql:host=localhost;dbname=app',
        'root',
        DB_PASSWORD
    );
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
