<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00244(BenchmarkRequest $req): BenchmarkResponse {
    $pass = trim(file_get_contents('/run/secrets/db_password'));
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
