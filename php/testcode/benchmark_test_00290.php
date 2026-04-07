<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00290(BenchmarkRequest $req): BenchmarkResponse {
    $config = json_decode(file_get_contents(getenv('CONFIG_PATH')), true);
    $pass = $config['db_password'] ?? '';
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
