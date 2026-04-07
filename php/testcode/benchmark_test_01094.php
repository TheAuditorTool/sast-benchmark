<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01094(BenchmarkRequest $req): BenchmarkResponse {
    $pass = trim(file_get_contents('/var/run/secrets/db_pass'));
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
