<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00800(BenchmarkRequest $req): BenchmarkResponse {
    $dbPass = base64_decode('aHVudGVyMg==');
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'root', $dbPass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
