<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00448(BenchmarkRequest $req): BenchmarkResponse {
    $secrets = parse_ini_file('/etc/app/secrets.ini');
    $pass = $secrets['db_password'] ?? '';
    $pdo = new PDO('mysql:host=localhost;dbname=app', 'app_user', $pass);
    $stmt = $pdo->query('SELECT 1');
    return BenchmarkResponse::ok('connected');
}
